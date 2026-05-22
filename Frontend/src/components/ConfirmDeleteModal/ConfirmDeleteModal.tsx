import { useState, useEffect } from 'react'
import { createPortal } from 'react-dom'
import styles from './ConfirmDeleteModal.module.scss'

interface Props {
  heading: string
  name: string
  onConfirm: () => void
  onCancel: () => void
}

export default function ConfirmDeleteModal({ heading, name, onConfirm, onCancel }: Props) {
  const [draft, setDraft] = useState('')
  const matches = draft === name

  useEffect(() => {
    const handler = (e: KeyboardEvent) => {
      if (e.key === 'Escape') onCancel()
    }
    window.addEventListener('keydown', handler)
    return () => window.removeEventListener('keydown', handler)
  }, [onCancel])

  return createPortal(
    <div className={styles.overlay} onMouseDown={(e) => { if (e.target === e.currentTarget) onCancel() }}>
      <div className={styles.modal}>
        <h3 className={styles.heading}>{heading}</h3>
        <p className={styles.warning}>
          Это действие необратимо. Введите <strong>{name}</strong>, чтобы подтвердить.
        </p>
        <input
          autoFocus
          className={styles.input}
          placeholder={name}
          value={draft}
          onChange={(e) => setDraft(e.target.value)}
          onKeyDown={(e) => { if (e.key === 'Enter' && matches) onConfirm() }}
        />
        <div className={styles.actions}>
          <button className={styles.cancelBtn} onClick={onCancel}>Отмена</button>
          <button className={styles.deleteBtn} onClick={onConfirm} disabled={!matches}>
            Удалить
          </button>
        </div>
      </div>
    </div>,
    document.body,
  )
}