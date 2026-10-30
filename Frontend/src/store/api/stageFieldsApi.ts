import {baseApi} from './baseApi'
import type {DetailedStage} from '../../types'

export const stageFieldsApi = baseApi.injectEndpoints({
  endpoints: (builder) => ({
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
        { type: 'Comment' as const, id: `${projectId}-${position}` },
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
        { type: 'Comment' as const, id: `${projectId}-${position}` },
        'Project',
        'Deadline',
      ],
    }),
    updateAdvanceCost: builder.mutation<void, { projectId: string; position: number; cost: number | null }>({
      query: ({ projectId, position, cost }) => ({
        url: `/projects/${projectId}/stages/${position}/advance-cost`,
        method: 'PATCH',
        body: { cost },
      }),
      invalidatesTags: (_r, _e, { projectId, position }) => [
        { type: 'Stage' as const, id: `detail-${projectId}-${position}` },
          {type: 'Stage' as const, id: projectId},
          {type: 'Comment' as const, id: `${projectId}-${position}`},
          'Project',
      ],
    }),
      updateSecondAdvanceCost: builder.mutation<void, { projectId: string; position: number; cost: number | null }>({
          query: ({projectId, position, cost}) => ({
              url: `/projects/${projectId}/stages/${position}/second-advance-cost`,
              method: 'PATCH',
              body: {cost},
          }),
          invalidatesTags: (_r, _e, {projectId, position}) => [
              {type: 'Stage' as const, id: `detail-${projectId}-${position}`},
        { type: 'Stage' as const, id: projectId },
        { type: 'Comment' as const, id: `${projectId}-${position}` },
        'Project',
      ],
    }),
    updateFinalCost: builder.mutation<void, { projectId: string; position: number; cost: number | null }>({
      query: ({ projectId, position, cost }) => ({
        url: `/projects/${projectId}/stages/${position}/final-cost`,
        method: 'PATCH',
        body: { cost },
      }),
      invalidatesTags: (_r, _e, { projectId, position }) => [
        { type: 'Stage' as const, id: `detail-${projectId}-${position}` },
        { type: 'Stage' as const, id: projectId },
        { type: 'Comment' as const, id: `${projectId}-${position}` },
        'Project',
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
        { type: 'Comment' as const, id: `${projectId}-${position}` },
        'Deadline',
        'Project',
      ],
    }),
    updateAdvanceConfirmed: builder.mutation<void, { projectId: string; position: number; confirmed: boolean }>({
      query: ({ projectId, position, confirmed }) => ({
        url: `/projects/${projectId}/stages/${position}/advance-confirmed`,
        method: 'PATCH',
        body: { confirmed },
      }),
        invalidatesTags: (_r, _e, {projectId, position}) => [
            {type: 'Stage' as const, id: `detail-${projectId}-${position}`},
            {type: 'Stage' as const, id: projectId},
            {type: 'Comment' as const, id: `${projectId}-${position}`},
            'Project',
        ],
    }),
      updateSecondAdvanceConfirmed: builder.mutation<void, {
          projectId: string;
          position: number;
          confirmed: boolean
      }>({
          query: ({projectId, position, confirmed}) => ({
              url: `/projects/${projectId}/stages/${position}/second-advance-confirmed`,
              method: 'PATCH',
              body: {confirmed},
      }),
      invalidatesTags: (_r, _e, { projectId, position }) => [
        { type: 'Stage' as const, id: `detail-${projectId}-${position}` },
        { type: 'Stage' as const, id: projectId },
        { type: 'Comment' as const, id: `${projectId}-${position}` },
        'Project',
      ],
    }),
    updateFinalConfirmed: builder.mutation<void, { projectId: string; position: number; confirmed: boolean }>({
      query: ({ projectId, position, confirmed }) => ({
        url: `/projects/${projectId}/stages/${position}/final-confirmed`,
        method: 'PATCH',
        body: { confirmed },
      }),
      invalidatesTags: (_r, _e, { projectId, position }) => [
        { type: 'Stage' as const, id: `detail-${projectId}-${position}` },
        { type: 'Stage' as const, id: projectId },
        { type: 'Comment' as const, id: `${projectId}-${position}` },
        'Project',
      ],
    }),

    // Sub-stage fields
    getDetailedSubStage: builder.query<DetailedStage, { projectId: string; parentPosition: number; position: number }>({
      query: ({ projectId, parentPosition, position }) => `/projects/${projectId}/stages/${parentPosition}/sub/${position}`,
      providesTags: (_r, _e, { projectId, parentPosition, position }) => [
        { type: 'Stage' as const, id: `detail-${projectId}-sub-${parentPosition}-${position}` },
      ],
    }),
    updateSubStageTitle: builder.mutation<void, { projectId: string; parentPosition: number; position: number; title: string }>({
      query: ({ projectId, parentPosition, position, title }) => ({
        url: `/projects/${projectId}/stages/${parentPosition}/sub/${position}/title`,
        method: 'PATCH',
        body: { title },
      }),
      invalidatesTags: (_r, _e, { projectId, parentPosition, position }) => [
        { type: 'Stage' as const, id: `detail-${projectId}-sub-${parentPosition}-${position}` },
        { type: 'Stage' as const, id: projectId },
        { type: 'Comment' as const, id: `${projectId}-sub-${parentPosition}-${position}` },
        'Project',
      ],
    }),
    updateSubStageDeadline: builder.mutation<void, { projectId: string; parentPosition: number; position: number; deadline: string | null }>({
      query: ({ projectId, parentPosition, position, deadline }) => ({
        url: `/projects/${projectId}/stages/${parentPosition}/sub/${position}/deadline`,
        method: 'PATCH',
        body: { deadline },
      }),
      invalidatesTags: (_r, _e, { projectId, parentPosition, position }) => [
        { type: 'Stage' as const, id: `detail-${projectId}-sub-${parentPosition}-${position}` },
        { type: 'Stage' as const, id: projectId },
        { type: 'Comment' as const, id: `${projectId}-sub-${parentPosition}-${position}` },
        'Project',
        'Deadline',
      ],
    }),
    updateSubStageAdvanceCost: builder.mutation<void, { projectId: string; parentPosition: number; position: number; cost: number | null }>({
      query: ({ projectId, parentPosition, position, cost }) => ({
        url: `/projects/${projectId}/stages/${parentPosition}/sub/${position}/advance-cost`,
        method: 'PATCH',
        body: { cost },
      }),
        invalidatesTags: (_r, _e, {projectId, parentPosition, position}) => [
            {type: 'Stage' as const, id: `detail-${projectId}-sub-${parentPosition}-${position}`},
            {type: 'Stage' as const, id: projectId},
            {type: 'Comment' as const, id: `${projectId}-sub-${parentPosition}-${position}`},
            'Project',
        ],
    }),
      updateSubStageSecondAdvanceCost: builder.mutation<void, {
          projectId: string;
          parentPosition: number;
          position: number;
          cost: number | null
      }>({
          query: ({projectId, parentPosition, position, cost}) => ({
              url: `/projects/${projectId}/stages/${parentPosition}/sub/${position}/second-advance-cost`,
              method: 'PATCH',
              body: {cost},
      }),
      invalidatesTags: (_r, _e, { projectId, parentPosition, position }) => [
        { type: 'Stage' as const, id: `detail-${projectId}-sub-${parentPosition}-${position}` },
        { type: 'Stage' as const, id: projectId },
        { type: 'Comment' as const, id: `${projectId}-sub-${parentPosition}-${position}` },
        'Project',
      ],
    }),
    updateSubStageFinalCost: builder.mutation<void, { projectId: string; parentPosition: number; position: number; cost: number | null }>({
      query: ({ projectId, parentPosition, position, cost }) => ({
        url: `/projects/${projectId}/stages/${parentPosition}/sub/${position}/final-cost`,
        method: 'PATCH',
        body: { cost },
      }),
      invalidatesTags: (_r, _e, { projectId, parentPosition, position }) => [
        { type: 'Stage' as const, id: `detail-${projectId}-sub-${parentPosition}-${position}` },
        { type: 'Stage' as const, id: projectId },
        { type: 'Comment' as const, id: `${projectId}-sub-${parentPosition}-${position}` },
        'Project',
      ],
    }),
    updateSubStageGipConfirmed: builder.mutation<void, { projectId: string; parentPosition: number; position: number; confirmed: boolean }>({
      query: ({ projectId, parentPosition, position, confirmed }) => ({
        url: `/projects/${projectId}/stages/${parentPosition}/sub/${position}/gip-confirmed`,
        method: 'PATCH',
        body: { confirmed },
      }),
      invalidatesTags: (_r, _e, { projectId, parentPosition, position }) => [
        { type: 'Stage' as const, id: `detail-${projectId}-sub-${parentPosition}-${position}` },
        { type: 'Stage' as const, id: projectId },
        { type: 'Comment' as const, id: `${projectId}-sub-${parentPosition}-${position}` },
        'Deadline',
        'Project',
      ],
    }),
    updateSubStageAdvanceConfirmed: builder.mutation<void, { projectId: string; parentPosition: number; position: number; confirmed: boolean }>({
      query: ({ projectId, parentPosition, position, confirmed }) => ({
        url: `/projects/${projectId}/stages/${parentPosition}/sub/${position}/advance-confirmed`,
        method: 'PATCH',
        body: { confirmed },
      }),
        invalidatesTags: (_r, _e, {projectId, parentPosition, position}) => [
            {type: 'Stage' as const, id: `detail-${projectId}-sub-${parentPosition}-${position}`},
            {type: 'Stage' as const, id: projectId},
            {type: 'Comment' as const, id: `${projectId}-sub-${parentPosition}-${position}`},
            'Project',
        ],
    }),
      updateSubStageSecondAdvanceConfirmed: builder.mutation<void, {
          projectId: string;
          parentPosition: number;
          position: number;
          confirmed: boolean
      }>({
          query: ({projectId, parentPosition, position, confirmed}) => ({
              url: `/projects/${projectId}/stages/${parentPosition}/sub/${position}/second-advance-confirmed`,
              method: 'PATCH',
              body: {confirmed},
      }),
      invalidatesTags: (_r, _e, { projectId, parentPosition, position }) => [
        { type: 'Stage' as const, id: `detail-${projectId}-sub-${parentPosition}-${position}` },
        { type: 'Stage' as const, id: projectId },
        { type: 'Comment' as const, id: `${projectId}-sub-${parentPosition}-${position}` },
        'Project',
      ],
    }),
    updateSubStageFinalConfirmed: builder.mutation<void, { projectId: string; parentPosition: number; position: number; confirmed: boolean }>({
      query: ({ projectId, parentPosition, position, confirmed }) => ({
        url: `/projects/${projectId}/stages/${parentPosition}/sub/${position}/final-confirmed`,
        method: 'PATCH',
        body: { confirmed },
      }),
      invalidatesTags: (_r, _e, { projectId, parentPosition, position }) => [
        { type: 'Stage' as const, id: `detail-${projectId}-sub-${parentPosition}-${position}` },
        { type: 'Stage' as const, id: projectId },
        { type: 'Comment' as const, id: `${projectId}-sub-${parentPosition}-${position}` },
        'Project',
      ],
    }),
    addPaymentTranche: builder.mutation<
        { id: string },
        {
          projectId: string
          parentPosition?: number | null
          position: number
          kind: 'first_advance' | 'second_advance' | 'final'
          amount: number
        }
    >({
      query: ({projectId, parentPosition, position, kind, amount}) => {
        const endpoint =
            kind === 'first_advance'
                ? 'advance-tranches'
                : kind === 'second_advance'
                    ? 'second-advance-tranches'
                    : 'final-tranches'
        const url =
            parentPosition != null
                ? `/projects/${projectId}/stages/${parentPosition}/sub/${position}/${endpoint}`
                : `/projects/${projectId}/stages/${position}/${endpoint}`
        return {
          url,
          method: 'POST',
          body: {amount},
        }
      },
      invalidatesTags: (_r, _e, {projectId, parentPosition, position}) => [
        parentPosition != null
            ? {type: 'Stage' as const, id: `detail-${projectId}-sub-${parentPosition}-${position}`}
            : {type: 'Stage' as const, id: `detail-${projectId}-${position}`},
        {type: 'Stage' as const, id: projectId},
        {
          type: 'Comment' as const,
          id: parentPosition != null ? `${projectId}-sub-${parentPosition}-${position}` : `${projectId}-${position}`,
        },
        'Project',
      ],
    }),
    deletePaymentTranche: builder.mutation<
        void,
        {
          projectId: string
          parentPosition?: number | null
          position: number
          kind: 'first_advance' | 'second_advance' | 'final'
          trancheId: string
        }
    >({
      query: ({projectId, parentPosition, position, kind, trancheId}) => {
        const endpoint =
            kind === 'first_advance'
                ? 'advance-tranches'
                : kind === 'second_advance'
                    ? 'second-advance-tranches'
                    : 'final-tranches'
        const url =
            parentPosition != null
                ? `/projects/${projectId}/stages/${parentPosition}/sub/${position}/${endpoint}/${trancheId}`
                : `/projects/${projectId}/stages/${position}/${endpoint}/${trancheId}`
        return {
          url,
          method: 'DELETE',
        }
      },
      invalidatesTags: (_r, _e, {projectId, parentPosition, position}) => [
        parentPosition != null
            ? {type: 'Stage' as const, id: `detail-${projectId}-sub-${parentPosition}-${position}`}
            : {type: 'Stage' as const, id: `detail-${projectId}-${position}`},
        {type: 'Stage' as const, id: projectId},
        {
          type: 'Comment' as const,
          id: parentPosition != null ? `${projectId}-sub-${parentPosition}-${position}` : `${projectId}-${position}`,
        },
        'Project',
      ],
    }),
  }),
})

export const {
  useGetDetailedStageQuery,
  useUpdateStageTitleMutation,
  useUpdateStageDeadlineMutation,
  useUpdateAdvanceCostMutation,
    useUpdateSecondAdvanceCostMutation,
  useUpdateFinalCostMutation,
  useUpdateGipConfirmedMutation,
  useUpdateAdvanceConfirmedMutation,
    useUpdateSecondAdvanceConfirmedMutation,
  useUpdateFinalConfirmedMutation,
  useGetDetailedSubStageQuery,
  useUpdateSubStageTitleMutation,
  useUpdateSubStageDeadlineMutation,
  useUpdateSubStageAdvanceCostMutation,
    useUpdateSubStageSecondAdvanceCostMutation,
  useUpdateSubStageFinalCostMutation,
  useUpdateSubStageGipConfirmedMutation,
  useUpdateSubStageAdvanceConfirmedMutation,
    useUpdateSubStageSecondAdvanceConfirmedMutation,
  useUpdateSubStageFinalConfirmedMutation,
  useAddPaymentTrancheMutation,
  useDeletePaymentTrancheMutation,
} = stageFieldsApi
