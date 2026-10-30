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
                                step,
                                validate,
                                stripSpaces = false,
}: {
  label: string
  displayValue: string
  rawValue: string
  onSave: (value: string) => Promise<void>
  type?: 'text' | 'number' | 'date'
  multiline?: boolean
  step?: string
  validate?: (value: string) => string | null
  stripSpaces?: boolean
}) {
  const [editing, setEditing] = useState(false)
  const [draft, setDraft] = useState('')
  const [error, setError] = useState<string | null>(null)
  const cancelled = useRef(false)

  const shouldStrip = stripSpaces || validate === validateCost

  const startEdit = () => {
    setDraft(shouldStrip ? rawValue.replace(/\s/g, '') : rawValue)
    setError(null)
    setEditing(true)
  }

  const handleChange = (val: string) => {
    const next = shouldStrip ? val.replace(/\s/g, '') : val
    setDraft(next)
    if (validate) {
      setError(validate(next))
    } else {
      setError(null)
    }
  }

  const handleBlur = async () => {
    if (cancelled.current) {
      cancelled.current = false
      setError(null)
      setEditing(false)
      return
    }
    if (validate) {
      const err = validate(draft)
      if (err) {
        setError(err)
        return
      }
    }
    setError(null)
    try {
      await onSave(draft)
      setEditing(false)
    } catch (e: any) {
      const msg = e?.data?.message || e?.message || 'Ошибка при сохранении'
      setError(typeof msg === 'string' ? msg : 'Ошибка при сохранении')
    }
  }

  const handleKeyDown = (e: React.KeyboardEvent<HTMLInputElement | HTMLTextAreaElement>) => {
    if (e.key === 'Enter' && !multiline) {
      if (validate) {
        const err = validate(draft)
        if (err) {
          setError(err)
          e.preventDefault()
          return
        }
      }
      ;(e.target as HTMLElement).blur()
    }
    if (e.key === 'Escape') {
      cancelled.current = true
      setError(null)
      setEditing(false)
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
            className={`${styles.fieldInput} ${error ? styles.fieldInputError : ''}`}
            value={draft}
            onChange={(e) => handleChange(e.target.value)}
            onBlur={handleBlur}
            onKeyDown={handleKeyDown}
            rows={3}
          />
        ) : (
          <input
            autoFocus
            type={type}
            step={step}
            className={`${styles.fieldInput} ${error ? styles.fieldInputError : ''}`}
            value={draft}
            onChange={(e) => handleChange(e.target.value)}
            onBlur={handleBlur}
            onKeyDown={handleKeyDown}
          />
        )
      ) : (
        <span className={styles.fieldValue}>{displayValue}</span>
      )}
      {error && <span className={styles.fieldErrorText}>{error}</span>}
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

export function formatNumber(value: number | string | null | undefined): string {
  if (value == null || value === '') return ''
  const num = typeof value === 'string' ? parseFloat(value.replace(/\s/g, '').replace(',', '.')) : value
  if (isNaN(num)) return ''
  const isNegative = num < 0
  const abs = Math.abs(num)
  const totalKopecks = Math.round(abs * 100)
  const rubles = Math.floor(totalKopecks / 100)
  const kopecks = totalKopecks % 100
  const rublesFormatted = rubles.toString().replace(/\B(?=(\d{3})+(?!\d))/g, ' ')
  if (kopecks > 0) {
    const kopecksFormatted = kopecks.toString().padStart(2, '0')
    return `${isNegative ? '-' : ''}${rublesFormatted},${kopecksFormatted}`
  }
  return `${isNegative ? '-' : ''}${rublesFormatted}`
}

export function formatRawCost(value: number | string | null | undefined): string {
  return formatNumber(value).replace(/\s/g, '')
}

export function validateCost(value: string): string | null {
  const trimmed = value.trim()
  if (!trimmed) return null

  const cleaned = trimmed.replace(/\s/g, '')

  if (!/^\d+([.,]\d+)?$/.test(cleaned)) {
    return 'Введите корректное неотрицательное число'
  }

  const parts = cleaned.split(/[.,]/)
  if (parts.length > 2) {
    return 'Допустим только один десятичный разделитель'
  }

  if (parts.length === 2 && parts[1].length > 2) {
    return 'После запятой не должно быть более двух знаков'
  }

  const num = parseFloat(cleaned.replace(',', '.'))
  if (isNaN(num)) {
    return 'Некорректное число'
  }

  if (num < 0) {
    return 'Сумма не может быть отрицательной'
  }

  return null
}

export function parseCost(value: string | number | null | undefined): number | null {
  if (value == null) return null
  if (typeof value === 'number') return isNaN(value) ? null : Math.round(value * 100) / 100
  const trimmed = String(value).trim()
  if (!trimmed) return null
  const cleaned = trimmed.replace(/\s/g, '').replace(',', '.')
  const num = parseFloat(cleaned)
  if (isNaN(num)) return null
  return Math.round(num * 100) / 100
}
