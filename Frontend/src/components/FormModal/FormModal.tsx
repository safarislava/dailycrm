import React, { useEffect } from 'react'
import { createPortal } from 'react-dom'
import styles from './FormModal.module.scss'

interface Props {
  heading: string
  onClose: () => void
  onSubmit: (e: React.FormEvent) => void
  loading: boolean
  error: string | null
  submitLabel: string
  children: React.ReactNode
}

export default function FormModal({ heading, onClose, onSubmit, loading, error, submitLabel, children }: Props) {
  useEffect(() => {
    const handler = (e: KeyboardEvent) => { if (e.key === 'Escape') onClose() }
    window.addEventListener('keydown', handler)
    return () => window.removeEventListener('keydown', handler)
  }, [onClose])

  return createPortal(
    <div className={styles.overlay} onMouseDown={(e) => { if (e.target === e.currentTarget) onClose() }}>
      <form className={styles.modal} onSubmit={onSubmit}>
        <h3 className={styles.heading}>{heading}</h3>
        <div className={styles.body}>{children}</div>
        {error && <p className={styles.error}>{error}</p>}
        <div className={styles.actions}>
          <button type="button" className={styles.cancelBtn} onClick={onClose}>Отмена</button>
          <button type="submit" className={styles.submitBtn} disabled={loading}>
            {loading ? '…' : submitLabel}
          </button>
        </div>
      </form>
    </div>,
    document.body,
  )
}