import React from 'react'
import {useDispatch} from 'react-redux'
import type {AppDispatch} from '../../../store'
import {selectStage} from '../../../store/uiSlice'
import {
  useGetDetailedStageQuery,
  useGetDetailedSubStageQuery,
  useListActsQuery,
  useListAttachmentsQuery,
  useListCommentsQuery,
  useListPinnedCommentsQuery,
  useListPinnedSubStageCommentsQuery,
  useListSubStageActsQuery,
  useListSubStageAttachmentsQuery,
  useListSubStageCommentsQuery,
  useUpdateAdvanceConfirmedMutation,
  useUpdateAdvanceCostMutation,
  useUpdateFinalConfirmedMutation,
  useUpdateFinalCostMutation,
  useUpdateGipConfirmedMutation,
  useUpdateStageDeadlineMutation,
  useUpdateStageTitleMutation,
  useUpdateSubStageAdvanceConfirmedMutation,
  useUpdateSubStageAdvanceCostMutation,
  useUpdateSubStageDeadlineMutation,
  useUpdateSubStageFinalConfirmedMutation,
  useUpdateSubStageFinalCostMutation,
  useUpdateSubStageGipConfirmedMutation,
  useUpdateSubStageTitleMutation,
} from '../../../store/crmApi'
import StageHeader from './stageDetails/StageHeader'
import StageFinancials from './stageDetails/StageFinancials'
import StageActsSection from './stageDetails/StageActsSection'
import StageAttachmentsSection from './stageDetails/StageAttachmentsSection'
import StageCommentsSection from './stageDetails/StageCommentsSection'
import {parseCost, validateCost} from './Helpers'
import styles from '../MainPanel.module.scss'

interface StageDetailsSidebarProps {
  projectId: string
  selectedStage: { parentPosition: number; position: number }
  isSub: boolean
  setPendingDelete: (deleteObj: any) => void
}

export default function StageDetailsSidebar({
  projectId,
  selectedStage,
  isSub,
  setPendingDelete,
}: StageDetailsSidebarProps) {
  const dispatch = useDispatch<AppDispatch>()

  // Queries
  const { data: topDetail, isLoading: topDetailLoading } = useGetDetailedStageQuery(
    { projectId, position: selectedStage.position },
    { skip: isSub },
  )
  const { data: subDetail, isLoading: subDetailLoading } = useGetDetailedSubStageQuery(
    { projectId, parentPosition: selectedStage.parentPosition, position: selectedStage.position },
    { skip: !isSub },
  )
  const detail = isSub ? subDetail : topDetail
  const detailLoading = isSub ? subDetailLoading : topDetailLoading

  const actArgs = { projectId, position: selectedStage.position }
  const subActArgs = { projectId, parentPosition: selectedStage.parentPosition, position: selectedStage.position }

  const { data: topActs = [] } = useListActsQuery(actArgs, { skip: isSub })
  const { data: subActs = [] } = useListSubStageActsQuery(subActArgs, { skip: !isSub })
  const acts = isSub ? subActs : topActs

  const { data: topAttachments = [] } = useListAttachmentsQuery(actArgs, { skip: isSub })
  const { data: subAttachments = [] } = useListSubStageAttachmentsQuery(subActArgs, { skip: !isSub })
  const attachments = isSub ? subAttachments : topAttachments

  const { data: topComments = [] } = useListCommentsQuery(actArgs, { skip: isSub })
  const { data: subComments = [] } = useListSubStageCommentsQuery(subActArgs, { skip: !isSub })
  const comments = isSub ? subComments : topComments

  const { data: topPinnedComments = [] } = useListPinnedCommentsQuery(actArgs, { skip: isSub })
  const { data: subPinnedComments = [] } = useListPinnedSubStageCommentsQuery(subActArgs, { skip: !isSub })
  const initialPinnedComments = isSub ? subPinnedComments : topPinnedComments

  // Field Mutations
  const [updateTopTitle] = useUpdateStageTitleMutation()
  const [updateTopDeadline] = useUpdateStageDeadlineMutation()
  const [updateTopAdvanceCost] = useUpdateAdvanceCostMutation()
  const [updateTopFinalCost] = useUpdateFinalCostMutation()
  const [updateTopGip] = useUpdateGipConfirmedMutation()
  const [updateTopAdvanceConfirmed] = useUpdateAdvanceConfirmedMutation()
  const [updateTopFinalConfirmed] = useUpdateFinalConfirmedMutation()

  const [updateSubTitle] = useUpdateSubStageTitleMutation()
  const [updateSubDeadline] = useUpdateSubStageDeadlineMutation()
  const [updateSubAdvanceCost] = useUpdateSubStageAdvanceCostMutation()
  const [updateSubFinalCost] = useUpdateSubStageFinalCostMutation()
  const [updateSubGip] = useUpdateSubStageGipConfirmedMutation()
  const [updateSubAdvanceConfirmed] = useUpdateSubStageAdvanceConfirmedMutation()
  const [updateSubFinalConfirmed] = useUpdateSubStageFinalConfirmedMutation()

  // Handlers
  const handleUpdateTitle = async (v: string) => {
    if (!v.trim()) return
    if (isSub) {
      await updateSubTitle({ projectId, parentPosition: selectedStage.parentPosition, position: selectedStage.position, title: v.trim() })
    } else {
      await updateTopTitle({ projectId, position: selectedStage.position, title: v.trim() })
    }
  }

  const handleUpdateDeadline = async (v: string) => {
    const deadline = v ? `${v}T00:00:00Z` : null
    if (isSub) {
      await updateSubDeadline({ projectId, parentPosition: selectedStage.parentPosition, position: selectedStage.position, deadline })
    } else {
      await updateTopDeadline({ projectId, position: selectedStage.position, deadline })
    }
  }

  const handleUpdateAdvanceCost = async (v: string) => {
    const err = validateCost(v)
    if (err) throw new Error(err)
    const cost = parseCost(v)
    if (isSub) {
      await updateSubAdvanceCost({
        projectId,
        parentPosition: selectedStage.parentPosition,
        position: selectedStage.position,
        cost
      }).unwrap()
    } else {
      await updateTopAdvanceCost({projectId, position: selectedStage.position, cost}).unwrap()
    }
  }

  const handleUpdateFinalCost = async (v: string) => {
    const err = validateCost(v)
    if (err) throw new Error(err)
    const cost = parseCost(v)
    if (isSub) {
      await updateSubFinalCost({
        projectId,
        parentPosition: selectedStage.parentPosition,
        position: selectedStage.position,
        cost
      }).unwrap()
    } else {
      await updateTopFinalCost({projectId, position: selectedStage.position, cost}).unwrap()
    }
  }

  const handleToggleGip = async () => {
    if (!detail) return
    if (isSub) {
      await updateSubGip({ projectId, parentPosition: selectedStage.parentPosition, position: selectedStage.position, confirmed: !detail.gip_confirmed })
    } else {
      await updateTopGip({ projectId, position: selectedStage.position, confirmed: !detail.gip_confirmed })
    }
  }

  const handleToggleAdvancePayment = async () => {
    if (!detail) return
    if (isSub) {
      await updateSubAdvanceConfirmed({ projectId, parentPosition: selectedStage.parentPosition, position: selectedStage.position, confirmed: !detail.advance_confirmed })
    } else {
      await updateTopAdvanceConfirmed({ projectId, position: selectedStage.position, confirmed: !detail.advance_confirmed })
    }
  }

  const handleToggleFinalPayment = async () => {
    if (!detail) return
    if (isSub) {
      await updateSubFinalConfirmed({ projectId, parentPosition: selectedStage.parentPosition, position: selectedStage.position, confirmed: !detail.final_confirmed })
    } else {
      await updateTopFinalConfirmed({ projectId, position: selectedStage.position, confirmed: !detail.final_confirmed })
    }
  }

  return (
    <>
      <StageHeader
        isSub={isSub}
        selectedStage={selectedStage}
        detail={detail}
        onBack={() => dispatch(selectStage(null))}
        setPendingDelete={setPendingDelete}
      />

      <div className={styles.detailScroll}>
        {detailLoading && <div className={styles.loading}>Загрузка…</div>}
        {!detailLoading && detail && (
          <div className={styles.detailCard}>
            <StageFinancials
              detail={detail}
              onUpdateTitle={handleUpdateTitle}
              onUpdateDeadline={handleUpdateDeadline}
              onToggleGip={handleToggleGip}
              onUpdateAdvanceCost={handleUpdateAdvanceCost}
              onToggleAdvancePayment={handleToggleAdvancePayment}
              onUpdateFinalCost={handleUpdateFinalCost}
              onToggleFinalPayment={handleToggleFinalPayment}
            />

            <StageActsSection
              projectId={projectId}
              selectedStage={selectedStage}
              isSub={isSub}
              acts={acts}
            />

            <StageAttachmentsSection
              projectId={projectId}
              selectedStage={selectedStage}
              isSub={isSub}
              attachments={attachments}
            />

            <StageCommentsSection
              projectId={projectId}
              selectedStage={selectedStage}
              isSub={isSub}
              comments={comments}
              initialPinnedComments={initialPinnedComments}
            />
          </div>
        )}
        {!detailLoading && !detail && <div className={styles.loading}>Нет данных</div>}
      </div>
    </>
  )
}
