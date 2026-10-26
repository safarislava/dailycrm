import React, {useRef, useState} from 'react'
import {store} from '../../../store'
import {PencilIcon} from '../../ui/icons'
import styles from '../MainPanel.module.scss'

export * from '../../ui/icons'

// ── InlineEdit ─────────────────────────────────────────────
export function InlineEdit({
  value,
  onSave,
  className,
}: {
  value: string
  onSave: (value: string) => Promise<void>
  className?: string
}) {
  const [editing, setEditing] = useState(false)
  const [draft, setDraft] = useState('')
  const cancelled = useRef(false)

  const startEdit = () => {
    setDraft(value)
    setEditing(true)
  }

  const handleBlur = async () => {
    if (cancelled.current) {
      cancelled.current = false
      return
    }
    setEditing(false)
    await onSave(draft)
  }

  const handleKeyDown = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === 'Enter') (e.target as HTMLElement).blur()
    if (e.key === 'Escape') {
      cancelled.current = true
      ;(e.target as HTMLElement).blur()
    }
  }

  return editing ? (
    <input
      autoFocus
      className={`${className ?? ''} ${styles.inlineInput}`}
      value={draft}
      onChange={(e) => setDraft(e.target.value)}
      onBlur={handleBlur}
      onKeyDown={handleKeyDown}
    />
  ) : (
    <span
      className={`${className ?? ''} ${styles.inlineValue}`}
      onClick={startEdit}
      title="Переименовать"
    >
      {value}
      <PencilIcon />
    </span>
  )
}

// ── EditableField ──────────────────────────────────────────
export function EditableField({
  label,
  displayValue,
  rawValue,
  onSave,
  type = 'text',
  multiline = false,
}: {
  label: string
  displayValue: string
  rawValue: string
  onSave: (value: string) => Promise<void>
  type?: 'text' | 'number' | 'date'
  multiline?: boolean
}) {
  const [editing, setEditing] = useState(false)
  const [draft, setDraft] = useState('')
  const cancelled = useRef(false)

  const startEdit = () => {
    setDraft(rawValue)
    setEditing(true)
  }

  const handleBlur = async () => {
    if (cancelled.current) {
      cancelled.current = false
      return
    }
    setEditing(false)
    await onSave(draft)
  }

  const handleKeyDown = (e: React.KeyboardEvent<HTMLInputElement | HTMLTextAreaElement>) => {
    if (e.key === 'Enter' && !multiline) (e.target as HTMLElement).blur()
    if (e.key === 'Escape') {
      cancelled.current = true
      ;(e.target as HTMLElement).blur()
    }
  }

  return (
    <div
      className={`${styles.field} ${styles.fieldEditable}`}
      onClick={!editing ? startEdit : undefined}
    >
      <span className={styles.fieldLabel}>{label}</span>
      {editing ? (
        multiline ? (
          <textarea
            autoFocus
            className={styles.fieldInput}
            value={draft}
            onChange={(e) => setDraft(e.target.value)}
            onBlur={handleBlur}
            onKeyDown={handleKeyDown}
            rows={3}
          />
        ) : (
          <input
            autoFocus
            type={type}
            className={styles.fieldInput}
            value={draft}
            onChange={(e) => setDraft(e.target.value)}
            onBlur={handleBlur}
            onKeyDown={handleKeyDown}
          />
        )
      ) : (
        <span className={styles.fieldValue}>{displayValue}</span>
      )}
    </div>
  )
}

// ── Helpers ────────────────────────────────────────────────
export async function downloadFile(url: string, filename: string) {
  const token = store.getState().auth.accessToken
  const res = await fetch(url, { headers: token ? { Authorization: `Bearer ${token}` } : {} })
  if (!res.ok) return
  const blob = await res.blob()
  const blobUrl = URL.createObjectURL(blob)
  const a = document.createElement('a')
  a.href = blobUrl
  a.download = filename
  document.body.appendChild(a)
  a.click()
  document.body.removeChild(a)
  setTimeout(() => URL.revokeObjectURL(blobUrl), 10000)
}

export function readFile(file: File): Promise<ArrayBuffer> {
  return new Promise((resolve, reject) => {
    const reader = new FileReader()
    reader.onload = () => resolve(reader.result as ArrayBuffer)
    reader.onerror = () => reject(reader.error)
    reader.readAsArrayBuffer(file)
  })
}

export function formatBytes(bytes: number): string {
  if (bytes < 1024) return `${bytes} B`
  if (bytes < 1_048_576) return `${(bytes / 1024).toFixed(1)} KB`
  return `${(bytes / 1_048_576).toFixed(1)} MB`
}

export function formatNumber(value: number | null | undefined): string {
  if (value == null || isNaN(value)) return ''
  const isNegative = value < 0
  const absStr = Math.abs(value).toString()
  const parts = absStr.split('.')
  parts[0] = parts[0].replace(/\B(?=(\d{3})+(?!\d))/g, ' ')
  return (isNegative ? '-' : '') + parts.join('.')
}
