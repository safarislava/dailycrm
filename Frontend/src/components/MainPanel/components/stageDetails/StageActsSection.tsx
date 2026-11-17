import React, { useState, useRef, useCallback } from 'react'
import type { Act } from '../../../../types'
import { useUploadActMutation, useDeleteActMutation, useUploadSubStageActMutation, useDeleteSubStageActMutation } from '../../../../store/crmApi'
import { downloadFile, readFile, formatBytes, PaperclipIcon, CloseIcon, FileIcon, SpinnerIcon } from '../Helpers'
import styles from '../../MainPanel.module.scss'

interface StageActsSectionProps {
  projectId: string
  selectedStage: { parentPosition: number; position: number }
  isSub: boolean
    isArchived?: boolean
  acts: Act[]
}

export default function StageActsSection({
  projectId,
  selectedStage,
  isSub,
                                             isArchived = false,
  acts,
}: StageActsSectionProps) {
  const [uploadTopAct, { isLoading: uploadingTopAct }] = useUploadActMutation()
  const [deleteTopAct] = useDeleteActMutation()
  const [uploadSubAct, { isLoading: uploadingSubAct }] = useUploadSubStageActMutation()
  const [deleteSubAct] = useDeleteSubStageActMutation()

  const uploadingAct = isSub ? uploadingSubAct : uploadingTopAct
  const actFileInputRef = useRef<HTMLInputElement>(null)
  const [actUploadError, setActUploadError] = useState<string | null>(null)

  const handleActFileChange = useCallback(
    async (e: React.ChangeEvent<HTMLInputElement>) => {
      const original = e.target.files?.[0]
      if (!original) return
      setActUploadError(null)
      const buffer = await readFile(original)
      const file = new File([buffer], original.name || 'act', { type: original.type || 'application/octet-stream' })
      let result
      if (isSub) {
        result = await uploadSubAct({ projectId, parentPosition: selectedStage.parentPosition, position: selectedStage.position, file })
      } else {
        result = await uploadTopAct({ projectId, position: selectedStage.position, file })
      }
      if (actFileInputRef.current) actFileInputRef.current.value = ''
      if ('error' in result) {
        const status = (result.error as { status?: number })?.status
        if (status === 413) setActUploadError('Файл слишком большой (макс. 50 МБ)')
        else setActUploadError('Не удалось загрузить акт')
      }
    },
    [projectId, selectedStage, isSub, uploadTopAct, uploadSubAct],
  )

  const handleDeleteAct = (actId: string) => {
    if (isSub) {
      deleteSubAct({ projectId, parentPosition: selectedStage.parentPosition, position: selectedStage.position, actId })
    } else {
      deleteTopAct({ projectId, position: selectedStage.position, actId })
    }
  }

  return (
    <div className={styles.attachmentsSection}>
      <div className={styles.attachmentsHeader}>
        <div className={styles.attachmentsHeaderLeft}>
          <span className={styles.attachmentsSectionLabel}>Акты</span>
          {acts.length > 0 && <span className={styles.completedBadge}>Акт загружен</span>}
        </div>
          {!isArchived && (
              <label className={`${styles.attachUploadBtn} ${uploadingAct ? styles.attachUploadDisabled : ''}`}>
                  {uploadingAct ? <SpinnerIcon/> : <PaperclipIcon/>}
                  {uploadingAct ? 'Загрузка…' : 'Загрузить акт'}
                  <input
                      ref={actFileInputRef}
                      type="file"
                      className={styles.fileInputHidden}
                      onChange={handleActFileChange}
                      disabled={uploadingAct}
                  />
              </label>
          )}
      </div>
      {actUploadError && <p className={styles.uploadError}>{actUploadError}</p>}
      {acts.length === 0 && !uploadingAct && <p className={styles.attachmentsEmpty}>Нет актов</p>}
      {acts.map((act) => (
        <div key={act.id} className={styles.attachItem}>
          <FileIcon mime={act.mime_type} />
          <div className={styles.attachInfo}>
            <button className={styles.attachName} onClick={() => downloadFile(act.download_url, act.filename)}>
              {act.filename}
            </button>
            <span className={styles.attachMeta}>{formatBytes(act.size_bytes)}</span>
          </div>
            {!isArchived && (
                <button className={styles.attachDeleteBtn} title="Удалить акт" onClick={() => handleDeleteAct(act.id)}>
                    <CloseIcon/>
                </button>
            )}
        </div>
      ))}
    </div>
  )
}

