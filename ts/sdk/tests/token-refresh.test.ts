import { afterEach, beforeEach, describe, it, mock } from 'node:test';
import assert from 'node:assert/strict';
import { inspect } from 'node:util';
import { AxiosError, AxiosHeaders } from 'axios';
import { BluefinProSdk } from '../src/sdk.js';

const DEAD_REFRESH_TOKEN = 'dead.refresh.token-SECRET';
const NEW_ACCESS_TOKEN = 'new.access.token';

function axiosError(status: number | undefined, code?: string): AxiosError {
  const config = {
    headers: new AxiosHeaders(),
    data: JSON.stringify({ refreshToken: DEAD_REFRESH_TOKEN }),
  };
  const response =
    status === undefined
      ? undefined
      : {
          status,
          statusText: '',
          headers: {},
          config,
          data: { message: 'refresh token is invalid' },
        };
  return new AxiosError(
    status === undefined ? 'Network Error' : `Request failed with status code ${status}`,
    code ?? (status === undefined ? 'ERR_NETWORK' : 'ERR_BAD_REQUEST'),
    config,
    undefined,
    response,
  );
}

function newSdk() {
  const signer = {
    getAddress: () => '0xabc',
    signLoginRequest: async () => 'signature',
  };
  const sdk = new BluefinProSdk(signer as never, 'testnet', {} as never, {
    refreshToken: DEAD_REFRESH_TOKEN,
    refreshTokenValidForSeconds: 30 * 24 * 3600,
  });
  const authApi = {
    authTokenRefreshPut: mock.fn(async () => {
      throw axiosError(401);
    }),
    authV2TokenPost: mock.fn(async () => ({
      data: {
        accessToken: NEW_ACCESS_TOKEN,
        accessTokenValidForSeconds: 300,
        refreshToken: 'new.refresh.token',
        refreshTokenValidForSeconds: 30 * 24 * 3600,
      },
    })),
  };
  (sdk as unknown as { authApi: typeof authApi }).authApi = authApi;
  return { sdk, authApi };
}

describe('token refresh rejected by auth-server', () => {
  let logged: string[];

  beforeEach(() => {
    logged = [];
    for (const level of ['log', 'warn', 'error'] as const) {
      mock.method(console, level, (...args: unknown[]) => {
        logged.push(args.map((a) => (typeof a === 'string' ? a : inspect(a, { depth: 10 }))).join(' '));
      });
    }
  });

  afterEach(() => {
    mock.restoreAll();
  });

  it('falls back to a full login without initialize()', async () => {
    const { sdk, authApi } = newSdk();

    assert.equal(await sdk.getAccessToken(), NEW_ACCESS_TOKEN);
    assert.equal(authApi.authTokenRefreshPut.mock.callCount(), 1);
    assert.equal(authApi.authV2TokenPost.mock.callCount(), 1);
  });

  it('does not retry the rejected refresh token on the next call', async () => {
    const { sdk, authApi } = newSdk();

    await sdk.getAccessToken();
    await sdk.getAccessToken();

    assert.equal(authApi.authTokenRefreshPut.mock.callCount(), 1);
  });

  it('falls back to a full login when connected, without logging out', async () => {
    const onLogout = mock.fn();
    const { sdk, authApi } = newSdk();
    const internals = sdk as unknown as { isConnected: boolean; onLogout?: () => void };
    internals.isConnected = true;
    internals.onLogout = onLogout;

    try {
      assert.equal(await sdk.getAccessToken(), NEW_ACCESS_TOKEN);
      assert.equal(authApi.authV2TokenPost.mock.callCount(), 1);
      assert.equal(onLogout.mock.callCount(), 0);
    } finally {
      await sdk.dispose();
    }
  });

  it('keeps the refresh token and rethrows on a network error', async () => {
    const { sdk, authApi } = newSdk();
    authApi.authTokenRefreshPut.mock.mockImplementation(async () => {
      throw axiosError(undefined);
    });

    await assert.rejects(sdk.getAccessToken(), /Token refresh failed/);
    assert.equal(authApi.authV2TokenPost.mock.callCount(), 0);
  });

  it('never logs the refresh token', async () => {
    const { sdk, authApi } = newSdk();
    authApi.authV2TokenPost.mock.mockImplementation(async () => {
      throw axiosError(500);
    });

    await assert.rejects(sdk.getAccessToken());
    assert.ok(logged.length > 0);
    for (const line of logged) {
      assert.ok(!line.includes(DEAD_REFRESH_TOKEN), `token leaked in log line: ${line.slice(0, 200)}`);
    }
  });
});
