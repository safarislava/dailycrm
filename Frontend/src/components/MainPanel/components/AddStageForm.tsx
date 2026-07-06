import React, { useState } from 'react'
import { useAppendStageMutation, useInsertStageMutation } from '../../../store/crmApi'
import { SendIcon } from './Helpers'
import styles from '../MainPanel.module.scss'

interface AddStageFormProps {
  projectId: string
}

export default function AddStageForm({ projectId }: AddStageFormProps) {
  const [title, setTitle]       = useState('')
  const [position, setPosition] = useState('')

  const [appendStage, { isLoading: appending }] = useAppendStageMutation()
  const [insertStage, { isLoading: inserting }] = useInsertStageMutation()
  const creating = appending || inserting

  const handleSend = async () => {
    const t = title.trim()
    const p = position.trim()
    if (!t || creating) return

    if (p === '') {
      await appendStage({ projectId, title: t })
    } else {
      await insertStage({ projectId, position: Number(p), title: t })
    }
    setTitle('')
    setPosition('')
  }

  const canSend = title.trim() !== '' && !creating

  return (
    <div className={styles.inputRow}>
      <input
        className={styles.posInput}
        type="number"
        placeholder="№"
        min={1}
        value={position}
        onChange={(e) => setPosition(e.target.value)}
        onKeyDown={(e) => { if (e.key === 'Enter') handleSend() }}
      />
      <input
        className={styles.textInput}
        placeholder="Новый этап…"
        value={title}
        onChange={(e) => setTitle(e.target.value)}
        onKeyDown={(e) => { if (e.key === 'Enter') handleSend() }}
      />
      <button className={styles.sendBtn} onClick={handleSend} disabled={!canSend}>
        <SendIcon />
      </button>
    </div>
  )
}
