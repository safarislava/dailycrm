export { baseApi as crmApi } from './api/baseApi'

export { stageFieldsApi } from './api/stageFieldsApi'
export { stageFilesApi } from './api/stageFilesApi'
export { stageCommentsApi } from './api/stageCommentsApi'

export {
  useRegisterMutation,
  useCreateInviteMutation,
  useLoginMutation,
  useRefreshMutation,
  useLogoutApiMutation,
} from './api/authApi'

export {
  useGetMeQuery,
  useUpdateUsernameMutation,
  useUpdatePasswordMutation,
  useUpdateEmailMutation,
  useUpdateNotificationsMutation,
  useUpdateRolesMutation,
} from './api/usersApi'

export {
  useGetDeadlinesQuery,
  useGetProjectsQuery,
  useCreateProjectMutation,
  useDeleteProjectMutation,
  useRenameProjectMutation,
} from './api/projectsApi'

export {
  useGetStagesQuery,
  useAppendStageMutation,
  useInsertStageMutation,
  useDeleteStageMutation,
  useReorderStageMutation,
  useAppendSubStageMutation,
  useDeleteSubStageMutation,
  useReorderSubStageMutation,
} from './api/stagesApi'

export {
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
} from './api/stageFieldsApi'

export {
  useListAttachmentsQuery,
  useUploadAttachmentMutation,
  useDeleteAttachmentMutation,
  useListActsQuery,
  useUploadActMutation,
  useDeleteActMutation,
  useListSubStageActsQuery,
  useUploadSubStageActMutation,
  useDeleteSubStageActMutation,
  useListSubStageAttachmentsQuery,
  useUploadSubStageAttachmentMutation,
  useDeleteSubStageAttachmentMutation,
} from './api/stageFilesApi'

export {
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
} from './api/stageCommentsApi'