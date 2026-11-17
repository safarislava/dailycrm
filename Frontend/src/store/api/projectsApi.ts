import { baseApi } from './baseApi'
import type { Project, StageWithProjectTitle } from '../../types'

export const projectsApi = baseApi.injectEndpoints({
  endpoints: (builder) => ({
    getDeadlines: builder.query<StageWithProjectTitle[], void>({
      query: () => '/projects/deadlines',
      providesTags: ['Deadline'],
    }),
    getProjects: builder.query<Project[], void>({
      query: () => '/projects',
      providesTags: ['Project'],
    }),
      getArchivedProjects: builder.query<Project[], void>({
          query: () => '/archive/projects',
          providesTags: ['ArchivedProject'],
      }),
    createProject: builder.mutation<void, { title: string }>({
      query: (body) => ({ url: '/projects', method: 'POST', body }),
      invalidatesTags: ['Project'],
    }),
      archiveProject: builder.mutation<void, string>({
          query: (id) => ({url: `/projects/${id}/archive`, method: 'POST'}),
          invalidatesTags: ['Project', 'ArchivedProject', 'Deadline'],
      }),
      restoreProject: builder.mutation<void, string>({
          query: (id) => ({url: `/archive/projects/${id}/restore`, method: 'POST'}),
          invalidatesTags: ['Project', 'ArchivedProject', 'Deadline'],
    }),
    renameProject: builder.mutation<void, { id: string; title: string }>({
      query: ({ id, title }) => ({
        url: `/projects/${id}/title`,
        method: 'PATCH',
        body: { title },
      }),
      invalidatesTags: ['Project'],
    }),
  }),
})

export const {
  useGetDeadlinesQuery,
  useGetProjectsQuery,
    useGetArchivedProjectsQuery,
  useCreateProjectMutation,
    useArchiveProjectMutation,
    useRestoreProjectMutation,
  useRenameProjectMutation,
} = projectsApi

