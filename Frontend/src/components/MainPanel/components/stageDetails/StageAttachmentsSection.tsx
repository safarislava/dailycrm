import React, { useState, useRef, useCallback } from 'react'
import type { Attachment } from '../../../../types'
import {
  useUploadAttachmentMutation,
  useDeleteAttachmentMutation,
  useUploadSubStageAttachmentMutation,
  useDeleteSubStageAttachmentMutation,
} from '../../../../store/crmApi'
import { downloadFile, readFile, formatBytes, PaperclipIcon, CloseIcon, FileIcon, SpinnerIcon } from '../Helpers'
import styles from '../../MainPanel.module.scss'

interface StageAttachmentsSectionProps {
  projectId: string
  selectedStage: { parentPosition: number; position: number }
  isSub: boolean
    isArchived?: boolean
  attachments: Attachment[]
}

export default function StageAttachmentsSection({
  projectId,
  selectedStage,
  isSub,
                                                    isArchived = false,
  attachments,
}: StageAttachmentsSectionProps) {
  const [uploadTopFile, { isLoading: uploadingTopFile }] = useUploadAttachmentMutation()
  const [deleteTopFile] = useDeleteAttachmentMutation()
  const [uploadSubFile, { isLoading: uploadingSubFile }] = useUploadSubStageAttachmentMutation()
  const [deleteSubFile] = useDeleteSubStageAttachmentMutation()

  const uploadingFile = isSub ? uploadingSubFile : uploadingTopFile
  const fileInputRef = useRef<HTMLInputElement>(null)
  const [uploadError, setUploadError] = useState<string | null>(null)

  const handleFileChange = useCallback(
    async (e: React.ChangeEvent<HTMLInputElement>) => {
      const original = e.target.files?.[0]
      if (!original) return
      setUploadError(null)
      const buffer = await readFile(original)
      const file = new File([buffer], original.name || 'file', { type: original.type || 'application/octet-stream' })
      let result
      if (isSub) {
        result = await uploadSubFile({ projectId, parentPosition: selectedStage.parentPosition, position: selectedStage.position, file })
      } else {
        result = await uploadTopFile({ projectId, position: selectedStage.position, file })
      }
      if (fileInputRef.current) fileInputRef.current.value = ''
      if ('error' in result) {
        const status = (result.error as { status?: number })?.status
        if (status === 413) setUploadError('Файл слишком большой (макс. 50 МБ)')
        else if (status === 400) setUploadError('Неверный формат запроса')
        else setUploadError('Не удалось загрузить файл')
      }
    },
    [projectId, selectedStage, isSub, uploadTopFile, uploadSubFile],
  )

  const handleDeleteAttachment = (attachmentId: string) => {
    if (isSub) {
      deleteSubFile({ projectId, parentPosition: selectedStage.parentPosition, position: selectedStage.position, attachmentId })
    } else {
      deleteTopFile({ projectId, position: selectedStage.position, attachmentId })
    }
  }

  return (
    <div className={styles.attachmentsSection}>
      <div className={styles.attachmentsHeader}>
        <span className={styles.attachmentsSectionLabel}>Файлы</span>
          {!isArchived && (
              <label
                  className={`${styles.attachUploadBtn} ${uploadingFile ? styles.attachUploadDisabled : ''}`}
                  title="Прикрепить файл"
              >
                  {uploadingFile ? <SpinnerIcon/> : <PaperclipIcon/>}
                  {uploadingFile ? 'Загрузка…' : 'Прикрепить'}
                  <input
                      ref={fileInputRef}
                      type="file"
                      className={styles.fileInputHidden}
                      onChange={handleFileChange}
                      disabled={uploadingFile}
                  />
              </label>
          )}
      </div>
      {uploadError && <p className={styles.uploadError}>{uploadError}</p>}
      {attachments.length === 0 && !uploadingFile && (
        <p className={styles.attachmentsEmpty}>Нет прикреплённых файлов</p>
      )}
      {attachments.map((a) => (
        <div key={a.id} className={styles.attachItem}>
          <FileIcon mime={a.mime_type} />
          <div className={styles.attachInfo}>
            <button className={styles.attachName} onClick={() => downloadFile(a.download_url, a.filename)}>
              {a.filename}
            </button>
            <span className={styles.attachMeta}>{formatBytes(a.size_bytes)}</span>
          </div>
            {!isArchived && (
                <button
                    className={styles.attachDeleteBtn}
                    title="Удалить файл"
                    onClick={() => handleDeleteAttachment(a.id)}
                >
                    <CloseIcon/>
                </button>
            )}
        </div>
      ))}
    </div>
  )
}

