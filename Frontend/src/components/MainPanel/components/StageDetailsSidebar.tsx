import React from 'react'
import {useDispatch} from 'react-redux'
import type {AppDispatch} from '../../../store'
import {selectStage} from '../../../store/uiSlice'
import {
    useAddPaymentTrancheMutation,
    useDeletePaymentTrancheMutation,
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
    useUpdateAdvanceCostMutation,
    useUpdateFinalCostMutation,
    useUpdateGipConfirmedMutation,
    useUpdateSecondAdvanceCostMutation,
    useUpdateStageDeadlineMutation,
    useUpdateStageTitleMutation,
    useUpdateSubStageAdvanceCostMutation,
    useUpdateSubStageDeadlineMutation,
    useUpdateSubStageFinalCostMutation,
    useUpdateSubStageGipConfirmedMutation,
    useUpdateSubStageSecondAdvanceCostMutation,
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
    const [updateTopSecondAdvanceCost] = useUpdateSecondAdvanceCostMutation()
  const [updateTopFinalCost] = useUpdateFinalCostMutation()
  const [updateTopGip] = useUpdateGipConfirmedMutation()

  const [updateSubTitle] = useUpdateSubStageTitleMutation()
  const [updateSubDeadline] = useUpdateSubStageDeadlineMutation()
  const [updateSubAdvanceCost] = useUpdateSubStageAdvanceCostMutation()
    const [updateSubSecondAdvanceCost] = useUpdateSubStageSecondAdvanceCostMutation()
  const [updateSubFinalCost] = useUpdateSubStageFinalCostMutation()
  const [updateSubGip] = useUpdateSubStageGipConfirmedMutation()

    const [addTranche] = useAddPaymentTrancheMutation()
    const [deleteTranche] = useDeletePaymentTrancheMutation()

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

    const handleUpdateSecondAdvanceCost = async (v: string) => {
        const err = validateCost(v)
        if (err) throw new Error(err)
        const cost = parseCost(v)
        if (isSub) {
            await updateSubSecondAdvanceCost({
                projectId,
                parentPosition: selectedStage.parentPosition,
                position: selectedStage.position,
                cost
            }).unwrap()
        } else {
            await updateTopSecondAdvanceCost({projectId, position: selectedStage.position, cost}).unwrap()
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

    const handleAddTranche = async (kind: 'first_advance' | 'second_advance' | 'final', amount: number) => {
        if (isSub) {
            await addTranche({
                projectId,
                parentPosition: selectedStage.parentPosition,
                position: selectedStage.position,
                kind,
                amount,
            }).unwrap()
        } else {
            await addTranche({
                projectId,
                position: selectedStage.position,
                kind,
                amount,
            }).unwrap()
        }
    }

    const handleDeleteTranche = async (kind: 'first_advance' | 'second_advance' | 'final', trancheId: string) => {
        if (isSub) {
            await deleteTranche({
                projectId,
                parentPosition: selectedStage.parentPosition,
                position: selectedStage.position,
                kind,
                trancheId,
            }).unwrap()
        } else {
            await deleteTranche({
                projectId,
                position: selectedStage.position,
                kind,
                trancheId,
            }).unwrap()
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
              onUpdateSecondAdvanceCost={handleUpdateSecondAdvanceCost}
              onUpdateFinalCost={handleUpdateFinalCost}
              onAddTranche={handleAddTranche}
              onDeleteTranche={handleDeleteTranche}
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
