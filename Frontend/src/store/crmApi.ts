import { createApi, fetchBaseQuery } from '@reduxjs/toolkit/query/react'
import type { Project, Stage, DetailedStage } from '../types'

export const crmApi = createApi({
  reducerPath: 'crmApi',
  baseQuery: fetchBaseQuery({ baseUrl: '/api' }),
  tagTypes: ['Project', 'Stage'],
  endpoints: (builder) => ({

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
      invalidatesTags: ['Project'],
    }),

    getStages: builder.query<Stage[], string>({
      query: (projectId) => `/projects/${projectId}/stages`,
      providesTags: (_r, _e, projectId) => [{ type: 'Stage', id: projectId }],
    }),
    createStage: builder.mutation<void, { projectId: string; position: number; title: string }>({
      query: ({ projectId, position, title }) => ({
        url: `/projects/${projectId}/stages`,
        method: 'POST',
        body: { position, title },
      }),
      invalidatesTags: (_r, _e, { projectId }) => [{ type: 'Stage', id: projectId }],
    }),
    deleteStage: builder.mutation<void, { projectId: string; stageId: string }>({
      query: ({ projectId, stageId }) => ({
        url: `/projects/${projectId}/stages/${stageId}`,
        method: 'DELETE',
      }),
      invalidatesTags: (_r, _e, { projectId }) => [{ type: 'Stage', id: projectId }],
    }),

    getDetailedStage: builder.query<DetailedStage, { projectId: string; stageId: string }>({
      query: ({ projectId, stageId }) => `/projects/${projectId}/stages/${stageId}`,
    }),

  }),
})

export const {
  useGetProjectsQuery,
  useCreateProjectMutation,
  useDeleteProjectMutation,
  useGetStagesQuery,
  useCreateStageMutation,
  useDeleteStageMutation,
  useGetDetailedStageQuery,
} = crmApi