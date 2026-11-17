import { baseApi } from './baseApi'
import type { Comment } from '../../types'

export const stageCommentsApi = baseApi.injectEndpoints({
  endpoints: (builder) => ({
      listComments: builder.query<Comment[], {
          projectId: string;
          position: number;
          before?: string;
          isArchived?: boolean
      }>({
          query: ({projectId, position, before, isArchived}) => {
              const base = isArchived ? `/archive/projects/${projectId}` : `/projects/${projectId}`
              return `${base}/stages/${position}/comments${before ? `?before=${before}` : ''}`
          },
      providesTags: (_r, _e, { projectId, position }) => [
        { type: 'Comment' as const, id: `${projectId}-${position}` },
      ],
    }),

    listPinnedComments: builder.query<Comment[], { projectId: string; position: number }>({
      query: ({ projectId, position }) =>
        `/projects/${projectId}/stages/${position}/comments/pinned`,
      providesTags: (_r, _e, { projectId, position }) => [
        { type: 'Comment' as const, id: `${projectId}-${position}` },
      ],
    }),
    addComment: builder.mutation<void, { projectId: string; position: number; text: string }>({
      query: ({ projectId, position, text }) => ({
        url: `/projects/${projectId}/stages/${position}/comments`,
        method: 'POST',
        body: { text },
      }),
      invalidatesTags: (_r, _e, { projectId, position }) => [
        { type: 'Comment' as const, id: `${projectId}-${position}` },
        'Project',
      ],
    }),
    deleteComment: builder.mutation<void, { projectId: string; position: number; commentId: string }>({
      query: ({ projectId, position, commentId }) => ({
        url: `/projects/${projectId}/stages/${position}/comments/${commentId}`,
        method: 'DELETE',
      }),
      invalidatesTags: (_r, _e, { projectId, position }) => [
        { type: 'Comment' as const, id: `${projectId}-${position}` },
      ],
    }),
    pinComment: builder.mutation<void, { projectId: string; position: number; commentId: string; pinned: boolean }>({
      query: ({ projectId, position, commentId, pinned }) => ({
        url: `/projects/${projectId}/stages/${position}/comments/${commentId}/pin`,
        method: 'PATCH',
        body: { pinned },
      }),
      invalidatesTags: (_r, _e, { projectId, position }) => [
        { type: 'Comment' as const, id: `${projectId}-${position}` },
      ],
    }),

    // Sub-stage comments
      listSubStageComments: builder.query<Comment[], {
          projectId: string;
          parentPosition: number;
          position: number;
          before?: string;
          isArchived?: boolean
      }>({
          query: ({projectId, parentPosition, position, before, isArchived}) => {
              const base = isArchived ? `/archive/projects/${projectId}` : `/projects/${projectId}`
              return `${base}/stages/${parentPosition}/sub/${position}/comments${before ? `?before=${before}` : ''}`
          },
      providesTags: (_r, _e, { projectId, parentPosition, position }) => [
        { type: 'Comment' as const, id: `${projectId}-sub-${parentPosition}-${position}` },
      ],
    }),

      listPinnedSubStageComments: builder.query<Comment[], { projectId: string; parentPosition: number; position: number }>({
      query: ({ projectId, parentPosition, position }) =>
        `/projects/${projectId}/stages/${parentPosition}/sub/${position}/comments/pinned`,
      providesTags: (_r, _e, { projectId, parentPosition, position }) => [
        { type: 'Comment' as const, id: `${projectId}-sub-${parentPosition}-${position}` },
      ],
    }),
    addSubStageComment: builder.mutation<void, { projectId: string; parentPosition: number; position: number; text: string }>({
      query: ({ projectId, parentPosition, position, text }) => ({
        url: `/projects/${projectId}/stages/${parentPosition}/sub/${position}/comments`,
        method: 'POST',
        body: { text },
      }),
      invalidatesTags: (_r, _e, { projectId, parentPosition, position }) => [
        { type: 'Comment' as const, id: `${projectId}-sub-${parentPosition}-${position}` },
        'Project',
      ],
    }),
    deleteSubStageComment: builder.mutation<void, { projectId: string; parentPosition: number; position: number; commentId: string }>({
      query: ({ projectId, parentPosition, position, commentId }) => ({
        url: `/projects/${projectId}/stages/${parentPosition}/sub/${position}/comments/${commentId}`,
        method: 'DELETE',
      }),
      invalidatesTags: (_r, _e, { projectId, parentPosition, position }) => [
        { type: 'Comment' as const, id: `${projectId}-sub-${parentPosition}-${position}` },
      ],
    }),
    pinSubStageComment: builder.mutation<void, { projectId: string; parentPosition: number; position: number; commentId: string; pinned: boolean }>({
      query: ({ projectId, parentPosition, position, commentId, pinned }) => ({
        url: `/projects/${projectId}/stages/${parentPosition}/sub/${position}/comments/${commentId}/pin`,
        method: 'PATCH',
        body: { pinned },
      }),
      invalidatesTags: (_r, _e, { projectId, parentPosition, position }) => [
        { type: 'Comment' as const, id: `${projectId}-sub-${parentPosition}-${position}` },
      ],
    }),
  }),
})

export const {
  useListCommentsQuery,
  useListPinnedCommentsQuery,
  useAddCommentMutation,
  useDeleteCommentMutation,
  usePinCommentMutation,
  useListSubStageCommentsQuery,
  useListPinnedSubStageCommentsQuery,
  useAddSubStageCommentMutation,
  useDeleteSubStageCommentMutation,
  usePinSubStageCommentMutation,
} = stageCommentsApi
