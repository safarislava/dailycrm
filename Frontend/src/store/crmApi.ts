import { createApi, fetchBaseQuery } from '@reduxjs/toolkit/query/react'
import type { Project, Stage, DetailedStage, DeadlineItem } from '../types'

export const crmApi = createApi({
  reducerPath: 'crmApi',
  baseQuery: fetchBaseQuery({ baseUrl: '/api' }),
  tagTypes: ['Project', 'Stage', 'Deadline'],
  endpoints: (builder) => ({

    getDeadlines: builder.query<DeadlineItem[], void>({
      query: () => '/deadlines',
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

    updateStageDescription: builder.mutation<void, { projectId: string; position: number; description: string | null }>({
      query: ({ projectId, position, description }) => ({
        url: `/projects/${projectId}/stages/${position}/description`,
        method: 'PATCH',
        body: { description },
      }),
      invalidatesTags: (_r, _e, { projectId, position }) => [
        { type: 'Stage' as const, id: `detail-${projectId}-${position}` },
        'Project',
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

    updateStageCompleted: builder.mutation<void, { projectId: string; position: number; completed: boolean }>({
      query: ({ projectId, position, completed }) => ({
        url: `/projects/${projectId}/stages/${position}/completed`,
        method: 'PATCH',
        body: { completed },
      }),
      invalidatesTags: (_r, _e, { projectId, position }) => [
        { type: 'Stage' as const, id: `detail-${projectId}-${position}` },
        { type: 'Stage' as const, id: projectId },
        'Deadline',
      ],
    }),

  }),
})

export const {
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
  useUpdateStageDescriptionMutation,
  useUpdateStageCostMutation,
  useUpdateStageCompletedMutation,
  useRenameProjectMutation,
} = crmApi