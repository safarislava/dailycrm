import { baseApi } from './baseApi'
import type { Stage } from '../../types'

export const stagesApi = baseApi.injectEndpoints({
  endpoints: (builder) => ({
      getStages: builder.query<Stage[], string | { projectId: string; isArchived?: boolean }>({
          query: (arg) => {
              const projectId = typeof arg === 'string' ? arg : arg.projectId
              const isArchived = typeof arg === 'object' && arg.isArchived
              return isArchived ? `/archive/projects/${projectId}/stages` : `/projects/${projectId}/stages`
          },
          providesTags: (_r, _e, arg) => {
              const projectId = typeof arg === 'string' ? arg : arg.projectId
              return [{type: 'Stage', id: projectId}]
          },
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
    reorderStage: builder.mutation<void, { projectId: string; position: number; to: number }>({
      query: ({ projectId, position, to }) => ({
        url: `/projects/${projectId}/stages/${position}/position`,
        method: 'PATCH',
        body: { to },
      }),
      invalidatesTags: (_r, _e, { projectId }) => [
        { type: 'Stage', id: projectId },
        'Project',
        'Deadline',
      ],
    }),
    appendSubStage: builder.mutation<void, { projectId: string; parentPosition: number; title: string }>({
      query: ({ projectId, parentPosition, title }) => ({
        url: `/projects/${projectId}/stages/${parentPosition}/sub`,
        method: 'POST',
        body: { title },
      }),
      invalidatesTags: (_r, _e, { projectId }) => [{ type: 'Stage', id: projectId }, 'Project'],
    }),
    deleteSubStage: builder.mutation<void, { projectId: string; parentPosition: number; position: number }>({
      query: ({ projectId, parentPosition, position }) => ({
        url: `/projects/${projectId}/stages/${parentPosition}/sub/${position}`,
        method: 'DELETE',
      }),
      invalidatesTags: (_r, _e, { projectId }) => [{ type: 'Stage', id: projectId }, 'Project', 'Deadline'],
    }),
    reorderSubStage: builder.mutation<void, { projectId: string; parentPosition: number; position: number; to: number }>({
      query: ({ projectId, parentPosition, position, to }) => ({
        url: `/projects/${projectId}/stages/${parentPosition}/sub/${position}/position`,
        method: 'PATCH',
        body: { to },
      }),
      invalidatesTags: (_r, _e, { projectId }) => [
        { type: 'Stage', id: projectId },
        'Project',
        'Deadline',
      ],
    }),
  }),
})

export const {
  useGetStagesQuery,
  useAppendStageMutation,
  useInsertStageMutation,
  useDeleteStageMutation,
  useReorderStageMutation,
  useAppendSubStageMutation,
  useDeleteSubStageMutation,
  useReorderSubStageMutation,
} = stagesApi
