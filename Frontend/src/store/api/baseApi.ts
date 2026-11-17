import {
  type BaseQueryFn,
  createApi,
  type FetchArgs,
  fetchBaseQuery,
  type FetchBaseQueryError,
} from '@reduxjs/toolkit/query/react'
import {logout, setAccessToken} from '../authSlice'

const baseQuery = fetchBaseQuery({
  baseUrl: '/api',
  credentials: 'include',
  prepareHeaders: (headers, { getState }) => {
    const token = (getState() as { auth: { accessToken: string | null } }).auth.accessToken
    if (token) headers.set('Authorization', `Bearer ${token}`)
    headers.set('Cache-Control', 'no-cache, no-store, must-revalidate')
    headers.set('Pragma', 'no-cache')
    headers.set('Expires', '0')
    return headers
  },
})

let isRefreshing = false
let refreshSubscribers: ((token: string | null) => void)[] = []

const subscribeTokenRefresh = (cb: (token: string | null) => void) => {
  refreshSubscribers.push(cb)
}

const onRefreshed = (token: string | null) => {
  refreshSubscribers.forEach((cb) => cb(token))
  refreshSubscribers = []
}

const baseQueryWithReauth: BaseQueryFn<string | FetchArgs, unknown, FetchBaseQueryError> = async (
  args,
  api,
  extraOptions,
) => {
  const url = typeof args === 'string' ? args : args.url
  if (url === '/auth/refresh') {
    if (isRefreshing) {
      const newToken = await new Promise<string | null>((resolve) => {
        subscribeTokenRefresh((token) => resolve(token))
      })
      if (newToken) {
        return { data: { access_token: newToken } }
      } else {
        return { error: { status: 401, data: { message: 'Token revoked or expired' } } }
      }
    }

    isRefreshing = true
    try {
      const result = await baseQuery(args, api, extraOptions)
      if (!result.error) {
        const { access_token } = result.data as { access_token: string }
        api.dispatch(setAccessToken(access_token))
        onRefreshed(access_token)
      } else {
        api.dispatch(logout())
        onRefreshed(null)
      }
      return result
    } finally {
      isRefreshing = false
    }
  }

  let result = await baseQuery(args, api, extraOptions)
  if (result.error?.status === 401) {
    if (!isRefreshing) {
      isRefreshing = true
      try {
        const refreshResult = await baseQuery(
          { url: '/auth/refresh', method: 'POST' },
          api,
          extraOptions,
        )
        if (refreshResult.data) {
          const { access_token } = refreshResult.data as { access_token: string }
          api.dispatch(setAccessToken(access_token))
          onRefreshed(access_token)
          result = await baseQuery(args, api, extraOptions)
        } else {
          api.dispatch(logout())
          onRefreshed(null)
        }
      } finally {
        isRefreshing = false
      }
    } else {
      const newToken = await new Promise<string | null>((resolve) => {
        subscribeTokenRefresh((token) => resolve(token))
      })
      if (newToken) {
        result = await baseQuery(args, api, extraOptions)
      }
    }
  }
  return result
}

export const baseApi = createApi({
  reducerPath: 'crmApi',
  baseQuery: baseQueryWithReauth,
    tagTypes: ['Project', 'ArchivedProject', 'Stage', 'Deadline', 'Me', 'Attachment', 'Act', 'Comment', 'AdminStatistics', 'AdminUsers', 'AdminInvitations', 'AdminLogs'],
  endpoints: () => ({}),
})

