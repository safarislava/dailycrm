import React, { useState, useRef } from 'react'
import { store } from '../../../store'
import styles from '../MainPanel.module.scss'

// ── Icons ──────────────────────────────────────────────────
export function ArrowLeftIcon() {
  return (
    <svg width="20" height="20" viewBox="0 0 24 24" fill="none">
      <path d="M15 18l-6-6 6-6" stroke="currentColor" strokeWidth="2.2" strokeLinecap="round" strokeLinejoin="round"/>
    </svg>
  )
}

export function TrashIcon() {
  return (
    <svg width="18" height="18" viewBox="0 0 24 24" fill="none">
      <polyline points="3 6 5 6 21 6" stroke="currentColor" strokeWidth="2" strokeLinecap="round"/>
      <path d="M19 6l-1 14H6L5 6" stroke="currentColor" strokeWidth="2" strokeLinecap="round"/>
      <path d="M10 11v6M14 11v6" stroke="currentColor" strokeWidth="2" strokeLinecap="round"/>
      <path d="M9 6V4h6v2" stroke="currentColor" strokeWidth="2" strokeLinecap="round"/>
    </svg>
  )
}

export function SendIcon() {
  return (
    <svg width="20" height="20" viewBox="0 0 24 24" fill="none">
      <path d="M22 2 11 13" stroke="currentColor" strokeWidth="2" strokeLinecap="round"/>
      <path d="M22 2 15 22 11 13 2 9l20-7Z" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"/>
    </svg>
  )
}

export function CloseIcon() {
  return (
    <svg width="10" height="10" viewBox="0 0 24 24" fill="none">
      <path d="M18 6 6 18M6 6l12 12" stroke="currentColor" strokeWidth="2.5" strokeLinecap="round"/>
    </svg>
  )
}

export function FolderIcon() {
  return (
    <svg width="64" height="64" viewBox="0 0 24 24" fill="none">
      <path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2Z"
        stroke="currentColor" strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round"/>
    </svg>
  )
}

export function PencilIcon() {
  return (
    <svg className={styles.pencilIcon} width="12" height="12" viewBox="0 0 24 24" fill="none">
      <path d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"/>
      <path d="M18.5 2.5a2.121 2.121 0 0 1 3 3L13 14l-4 1 1-4 8.5-8.5Z" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"/>
    </svg>
  )
}

export function ListIcon() {
  return (
    <svg width="40" height="40" viewBox="0 0 24 24" fill="none">
      <line x1="8" y1="6" x2="21" y2="6" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round"/>
      <line x1="8" y1="12" x2="21" y2="12" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round"/>
      <line x1="8" y1="18" x2="21" y2="18" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round"/>
      <circle cx="3" cy="6" r="1.2" fill="currentColor"/>
      <circle cx="3" cy="12" r="1.2" fill="currentColor"/>
      <circle cx="3" cy="18" r="1.2" fill="currentColor"/>
    </svg>
  )
}

export function CircleIcon() {
  return (
    <svg width="18" height="18" viewBox="0 0 24 24" fill="none">
      <circle cx="12" cy="12" r="9" stroke="currentColor" strokeWidth="2"/>
    </svg>
  )
}

export function CheckCircleIcon() {
  return (
    <svg width="18" height="18" viewBox="0 0 24 24" fill="none">
      <circle cx="12" cy="12" r="9" stroke="currentColor" strokeWidth="2"/>
      <path d="m8 12 3 3 5-5" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"/>
    </svg>
  )
}

export function PaperclipIcon() {
  return (
    <svg width="14" height="14" viewBox="0 0 24 24" fill="none">
      <path d="M21.44 11.05l-9.19 9.19a6 6 0 0 1-8.49-8.49l9.19-9.19a4 4 0 0 1 5.66 5.66l-9.2 9.19a2 2 0 0 1-2.83-2.83l8.49-8.48"
        stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"/>
    </svg>
  )
}

export function FileIcon({ mime }: { mime: string }) {
  const isImage = mime.startsWith('image/')
  const isPdf   = mime === 'application/pdf'
  const color   = isImage ? '#65aadd' : isPdf ? '#e53935' : '#708499'
  return (
    <svg width="28" height="28" viewBox="0 0 24 24" fill="none" style={{ flexShrink: 0, color }}>
      <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8Z" stroke="currentColor" strokeWidth="1.8" strokeLinecap="round" strokeLinejoin="round"/>
      <path d="M14 2v6h6" stroke="currentColor" strokeWidth="1.8" strokeLinecap="round" strokeLinejoin="round"/>
    </svg>
  )
}

export function SpinnerIcon() {
  return (
    <svg width="14" height="14" viewBox="0 0 24 24" fill="none" style={{ animation: 'spin 0.8s linear infinite' }}>
      <circle cx="12" cy="12" r="9" stroke="currentColor" strokeWidth="2.5" strokeDasharray="40 20" strokeLinecap="round"/>
    </svg>
  )
}

export function ChevronRightIcon() {
  return (
    <svg width="14" height="14" viewBox="0 0 24 24" fill="none">
      <path d="M9 18l6-6-6-6" stroke="currentColor" strokeWidth="2.2" strokeLinecap="round" strokeLinejoin="round"/>
    </svg>
  )
}

export function PlusIcon() {
  return (
    <svg width="12" height="12" viewBox="0 0 24 24" fill="none">
      <path d="M12 5v14M5 12h14" stroke="currentColor" strokeWidth="2.5" strokeLinecap="round"/>
    </svg>
  )
}

export function PinIcon({ filled = false, className }: { filled?: boolean; className?: string }) {
  return (
    <svg className={className} width="14" height="14" viewBox="0 0 24 24" fill={filled ? "currentColor" : "none"}>
      <path d="M16 12V4H8v8L6 14v2h5v6l1 1 1-1v-6h5v-2l-2-2Z" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"/>
    </svg>
  )
}

// ── InlineEdit ─────────────────────────────────────────────
export function InlineEdit({ value, onSave, className }: {
  value: string
  onSave: (value: string) => Promise<void>
  className?: string
}) {
  const [editing, setEditing] = useState(false)
  const [draft, setDraft]     = useState('')
  const cancelled             = useRef(false)

  const startEdit = () => { setDraft(value); setEditing(true) }

  const handleBlur = async () => {
    if (cancelled.current) { cancelled.current = false; return }
    setEditing(false)
    await onSave(draft)
  }

  const handleKeyDown = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === 'Enter')  (e.target as HTMLElement).blur()
    if (e.key === 'Escape') { cancelled.current = true; (e.target as HTMLElement).blur() }
  }

  return editing ? (
    <input autoFocus className={`${className ?? ''} ${styles.inlineInput}`}
      value={draft} onChange={(e) => setDraft(e.target.value)}
      onBlur={handleBlur} onKeyDown={handleKeyDown} />
  ) : (
    <span className={`${className ?? ''} ${styles.inlineValue}`} onClick={startEdit} title="Переименовать">
      {value}<PencilIcon />
    </span>
  )
}

// ── EditableField ──────────────────────────────────────────
export function EditableField({ label, displayValue, rawValue, onSave, type = 'text', multiline = false }: {
  label: string
  displayValue: string
  rawValue: string
  onSave: (value: string) => Promise<void>
  type?: 'text' | 'number' | 'date'
  multiline?: boolean
}) {
  const [editing, setEditing] = useState(false)
  const [draft, setDraft]     = useState('')
  const cancelled             = useRef(false)

  const startEdit = () => { setDraft(rawValue); setEditing(true) }

  const handleBlur = async () => {
    if (cancelled.current) { cancelled.current = false; return }
    setEditing(false)
    await onSave(draft)
  }

  const handleKeyDown = (e: React.KeyboardEvent<HTMLInputElement | HTMLTextAreaElement>) => {
    if (e.key === 'Enter' && !multiline) (e.target as HTMLElement).blur()
    if (e.key === 'Escape') { cancelled.current = true; (e.target as HTMLElement).blur() }
  }

  return (
    <div className={`${styles.field} ${styles.fieldEditable}`} onClick={!editing ? startEdit : undefined}>
      <span className={styles.fieldLabel}>{label}</span>
      {editing ? (
        multiline ? (
          <textarea autoFocus className={styles.fieldInput} value={draft}
            onChange={(e) => setDraft(e.target.value)} onBlur={handleBlur} onKeyDown={handleKeyDown} rows={3} />
        ) : (
          <input autoFocus type={type} className={styles.fieldInput} value={draft}
            onChange={(e) => setDraft(e.target.value)} onBlur={handleBlur} onKeyDown={handleKeyDown} />
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
    reader.onload  = () => resolve(reader.result as ArrayBuffer)
    reader.onerror = () => reject(reader.error)
    reader.readAsArrayBuffer(file)
  })
}

export function formatBytes(bytes: number): string {
  if (bytes < 1024)       return `${bytes} B`
  if (bytes < 1_048_576)  return `${(bytes / 1024).toFixed(1)} KB`
  return `${(bytes / 1_048_576).toFixed(1)} MB`
}
