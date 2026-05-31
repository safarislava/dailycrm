import {
  createApi,
  fetchBaseQuery,
  type BaseQueryFn,
  type FetchArgs,
  type FetchBaseQueryError,
} from '@reduxjs/toolkit/query/react'
import type { Project, Role, Stage, DetailedStage, StageWithProjectTitle, Attachment, Act } from '../types'
import { setAccessToken, setInitialized, logout } from './authSlice'

const baseQuery = fetchBaseQuery({
  baseUrl: '/api',
  credentials: 'include',
  prepareHeaders: (headers, { getState }) => {
    const token = (getState() as { auth: { accessToken: string | null } }).auth.accessToken
    if (token) headers.set('Authorization', `Bearer ${token}`)
    return headers
  },
})

const baseQueryWithReauth: BaseQueryFn<string | FetchArgs, unknown, FetchBaseQueryError> = async (
  args,
  api,
  extraOptions,
) => {
  let result = await baseQuery(args, api, extraOptions)
  if (result.error?.status === 401) {
    const refreshResult = await baseQuery(
      { url: '/auth/refresh', method: 'POST' },
      api,
      extraOptions,
    )
    if (refreshResult.data) {
      const { access_token } = refreshResult.data as { access_token: string }
      api.dispatch(setAccessToken(access_token))
      result = await baseQuery(args, api, extraOptions)
    } else {
      api.dispatch(logout())
    }
  }
  return result
}

export const crmApi = createApi({
  reducerPath: 'crmApi',
  baseQuery: baseQueryWithReauth,
  tagTypes: ['Project', 'Stage', 'Deadline', 'Me', 'Attachment', 'Act'],
  endpoints: (builder) => ({

    register: builder.mutation<void, { username: string; password: string; invite_token: string; email: string }>({
      query: (body) => ({ url: '/users', method: 'POST', body }),
    }),
    createInvite: builder.mutation<{ token: string }, void>({
      query: () => ({ url: '/invites', method: 'POST' }),
    }),

    getMe: builder.query<{ username: string; email: string; notifications_enabled: boolean; roles: Role[] }, void>({
      query: () => '/users/me',
      providesTags: ['Me'],
    }),
    updateUsername: builder.mutation<void, { username: string }>({
      query: (body) => ({ url: '/users/me/username', method: 'PATCH', body }),
      invalidatesTags: ['Me'],
    }),
    updatePassword: builder.mutation<void, { current_password: string; new_password: string }>({
      query: (body) => ({ url: '/users/me/password', method: 'PATCH', body }),
    }),
    updateEmail: builder.mutation<void, { email: string }>({
      query: (body) => ({ url: '/users/me/email', method: 'PATCH', body }),
      invalidatesTags: ['Me'],
    }),
    updateNotifications: builder.mutation<void, { enabled: boolean }>({
      query: (body) => ({ url: '/users/me/notifications', method: 'PATCH', body }),
      invalidatesTags: ['Me'],
    }),
    login: builder.mutation<{ access_token: string }, { username: string; password: string }>({
      query: (body) => ({ url: '/auth/login', method: 'POST', body }),
    }),
    refresh: builder.mutation<{ access_token: string }, void>({
      query: () => ({ url: '/auth/refresh', method: 'POST' }),
      onQueryStarted: async (_arg, { dispatch, queryFulfilled }) => {
        try {
          const { data } = await queryFulfilled
          dispatch(setAccessToken(data.access_token))
        } catch {
          dispatch(setInitialized())
        }
      },
    }),
    logoutApi: builder.mutation<void, void>({
      query: () => ({ url: '/auth/logout', method: 'POST' }),
      onQueryStarted: async (_arg, { dispatch, queryFulfilled }) => {
        await queryFulfilled.catch(() => {})
        dispatch(logout())
      },
    }),

    getDeadlines: builder.query<StageWithProjectTitle[], void>({
      query: () => '/projects/deadlines',
      providesTags: ['Deadline'],
    }),

    getProjects: builder.query<Project[], void>({
      query: () => '/projects',
      providesTags: ['Project'],
    }),
    createProject: builder.mutation<void, { title: string }>({
      query: (body) => ({ url: '/projects', method: 'POST', body }),
      invalidatesTags: ['Project'],
    }),
    deleteProject: builder.mutation<void, string>({
      query: (id) => ({ url: `/projects/${id}`, method: 'DELETE' }),
      invalidatesTags: ['Project', 'Deadline'],
    }),
    renameProject: builder.mutation<void, { id: string; title: string }>({
      query: ({ id, title }) => ({
        url: `/projects/${id}/title`,
        method: 'PATCH',
        body: { title },
      }),
      invalidatesTags: ['Project'],
    }),

    getStages: builder.query<Stage[], string>({
      query: (projectId) => `/projects/${projectId}/stages`,
      providesTags: (_r, _e, projectId) => [{ type: 'Stage', id: projectId }],
    }),
    appendStage: builder.mutation<void, { projectId: string; title: string }>({
      query: ({ projectId, title }) => ({
        url: `/projects/${projectId}/stages`,
        method: 'POST',
        body: { title },
      }),
      invalidatesTags: (_r, _e, { projectId }) => [{ type: 'Stage', id: projectId }, 'Project'],
    }),
    insertStage: builder.mutation<void, { projectId: string; position: number; title: string }>({
      query: ({ projectId, position, title }) => ({
        url: `/projects/${projectId}/stages/${position}`,
        method: 'POST',
        body: { title },
      }),
      invalidatesTags: (_r, _e, { projectId }) => [{ type: 'Stage', id: projectId }, 'Project'],
    }),
    deleteStage: builder.mutation<void, { projectId: string; position: number }>({
      query: ({ projectId, position }) => ({
        url: `/projects/${projectId}/stages/${position}`,
        method: 'DELETE',
      }),
      invalidatesTags: (_r, _e, { projectId }) => [{ type: 'Stage', id: projectId }, 'Project', 'Deadline'],
    }),

    getDetailedStage: builder.query<DetailedStage, { projectId: string; position: number }>({
      query: ({ projectId, position }) => `/projects/${projectId}/stages/${position}`,
      providesTags: (_r, _e, { projectId, position }) => [
        { type: 'Stage' as const, id: `detail-${projectId}-${position}` },
      ],
    }),

    updateStageTitle: builder.mutation<void, { projectId: string; position: number; title: string }>({
      query: ({ projectId, position, title }) => ({
        url: `/projects/${projectId}/stages/${position}/title`,
        method: 'PATCH',
        body: { title },
      }),
      invalidatesTags: (_r, _e, { projectId, position }) => [
        { type: 'Stage' as const, id: `detail-${projectId}-${position}` },
        { type: 'Stage' as const, id: projectId },
        'Project',
      ],
    }),

    updateStageDeadline: builder.mutation<void, { projectId: string; position: number; deadline: string | null }>({
      query: ({ projectId, position, deadline }) => ({
        url: `/projects/${projectId}/stages/${position}/deadline`,
        method: 'PATCH',
        body: { deadline },
      }),
      invalidatesTags: (_r, _e, { projectId, position }) => [
        { type: 'Stage' as const, id: `detail-${projectId}-${position}` },
        { type: 'Stage' as const, id: projectId },
        'Project',
        'Deadline',
      ],
    }),

    updateStageCost: builder.mutation<void, { projectId: string; position: number; cost: number | null }>({
      query: ({ projectId, position, cost }) => ({
        url: `/projects/${projectId}/stages/${position}/cost`,
        method: 'PATCH',
        body: { cost },
      }),
      invalidatesTags: (_r, _e, { projectId, position }) => [
        { type: 'Stage' as const, id: `detail-${projectId}-${position}` },
        'Project',
      ],
    }),

    listAttachments: builder.query<Attachment[], { projectId: string; position: number }>({
      query: ({ projectId, position }) => `/projects/${projectId}/stages/${position}/attachments`,
      providesTags: (_r, _e, { projectId, position }) => [
        { type: 'Attachment' as const, id: `${projectId}-${position}` },
      ],
    }),
    uploadAttachment: builder.mutation<{ id: string }, { projectId: string; position: number; file: File }>({
      query: ({ projectId, position, file }) => {
        const body = new FormData()
        body.append('file', file)
        return { url: `/projects/${projectId}/stages/${position}/attachments`, method: 'POST', body }
      },
      invalidatesTags: (_r, _e, { projectId, position }) => [
        { type: 'Attachment' as const, id: `${projectId}-${position}` },
      ],
    }),
    deleteAttachment: builder.mutation<void, { projectId: string; position: number; attachmentId: string }>({
      query: ({ projectId, position, attachmentId }) => ({
        url: `/projects/${projectId}/stages/${position}/attachments/${attachmentId}`,
        method: 'DELETE',
      }),
      invalidatesTags: (_r, _e, { projectId, position }) => [
        { type: 'Attachment' as const, id: `${projectId}-${position}` },
      ],
    }),

    updateGipConfirmed: builder.mutation<void, { projectId: string; position: number; confirmed: boolean }>({
      query: ({ projectId, position, confirmed }) => ({
        url: `/projects/${projectId}/stages/${position}/gip-confirmed`,
        method: 'PATCH',
        body: { confirmed },
      }),
      invalidatesTags: (_r, _e, { projectId, position }) => [
        { type: 'Stage' as const, id: `detail-${projectId}-${position}` },
        { type: 'Stage' as const, id: projectId },
        'Deadline',
        'Project',
      ],
    }),

    updatePaymentConfirmed: builder.mutation<void, { projectId: string; position: number; confirmed: boolean }>({
      query: ({ projectId, position, confirmed }) => ({
        url: `/projects/${projectId}/stages/${position}/payment-confirmed`,
        method: 'PATCH',
        body: { confirmed },
      }),
      invalidatesTags: (_r, _e, { projectId, position }) => [
        { type: 'Stage' as const, id: `detail-${projectId}-${position}` },
        { type: 'Stage' as const, id: projectId },
        'Project',
      ],
    }),

    listActs: builder.query<Act[], { projectId: string; position: number }>({
      query: ({ projectId, position }) => `/projects/${projectId}/stages/${position}/act`,
      providesTags: (_r, _e, { projectId, position }) => [
        { type: 'Act' as const, id: `${projectId}-${position}` },
      ],
    }),

    uploadAct: builder.mutation<void, { projectId: string; position: number; file: File }>({
      query: ({ projectId, position, file }) => {
        const body = new FormData()
        body.append('file', file)
        return { url: `/projects/${projectId}/stages/${position}/act`, method: 'POST', body }
      },
      invalidatesTags: (_r, _e, { projectId, position }) => [
        { type: 'Act' as const, id: `${projectId}-${position}` },
        { type: 'Stage' as const, id: `detail-${projectId}-${position}` },
        { type: 'Stage' as const, id: projectId },
        'Project',
      ],
    }),

    deleteAct: builder.mutation<void, { projectId: string; position: number; actId: string }>({
      query: ({ projectId, position, actId }) => ({
        url: `/projects/${projectId}/stages/${position}/act/${actId}`,
        method: 'DELETE',
      }),
      invalidatesTags: (_r, _e, { projectId, position }) => [
        { type: 'Act' as const, id: `${projectId}-${position}` },
        { type: 'Stage' as const, id: `detail-${projectId}-${position}` },
        { type: 'Stage' as const, id: projectId },
        'Project',
      ],
    }),

    updateRoles: builder.mutation<void, { roles: Role[] }>({
      query: (body) => ({ url: '/users/me/roles', method: 'PATCH', body }),
      invalidatesTags: ['Me'],
    }),

  }),
})

export const {
  useListActsQuery,
  useListAttachmentsQuery,
  useUploadAttachmentMutation,
  useDeleteAttachmentMutation,
  useRegisterMutation,
  useCreateInviteMutation,
  useLoginMutation,
  useGetMeQuery,
  useUpdateUsernameMutation,
  useUpdatePasswordMutation,
  useUpdateEmailMutation,
  useUpdateNotificationsMutation,
  useUpdateRolesMutation,
  useRefreshMutation,
  useLogoutApiMutation,
  useGetDeadlinesQuery,
  useGetProjectsQuery,
  useCreateProjectMutation,
  useDeleteProjectMutation,
  useGetStagesQuery,
  useAppendStageMutation,
  useInsertStageMutation,
  useDeleteStageMutation,
  useGetDetailedStageQuery,
  useUpdateStageTitleMutation,
  useUpdateStageDeadlineMutation,
  useUpdateStageCostMutation,
  useUpdateGipConfirmedMutation,
  useUpdatePaymentConfirmedMutation,
  useUploadActMutation,
  useDeleteActMutation,
  useRenameProjectMutation,
} = crmApi