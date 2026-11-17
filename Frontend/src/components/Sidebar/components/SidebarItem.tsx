import React from 'react'
import type {Project} from '../../../types'
import {RestoreIcon} from '../../ui/icons'
import styles from '../Sidebar.module.scss'

const AVATAR_COLORS = [
  '#e17076',
  '#7bc862',
  '#65aadd',
  '#a695e7',
  '#ee7aae',
  '#faa774',
  '#6ec9cb',
]

const avatarColor = (title: string) =>
  AVATAR_COLORS[title.charCodeAt(0) % AVATAR_COLORS.length]

function formatUpdatedAt(iso: string): string {
  const date = new Date(iso)
  const now = new Date()
  const diffMs = now.getTime() - date.getTime()
  const diffMin = Math.floor(diffMs / 60_000)
  if (diffMin < 1) return 'только что'
  if (diffMin < 60) return `${diffMin} мин. назад`
  const diffH = Math.floor(diffMin / 60)
  if (diffH < 24) return `${diffH} ч. назад`
  const diffD = Math.floor(diffH / 24)
  if (diffD < 7) return `${diffD} дн. назад`
  return date.toLocaleDateString('ru-RU', { day: 'numeric', month: 'short' })
}

interface SidebarItemProps {
  project: Project
  isSelected: boolean
  isArchived?: boolean
  onSelect: () => void
  onRestore?: (e: React.MouseEvent) => void
}

export default function SidebarItem({
  project,
  isSelected,
                                      isArchived = false,
  onSelect,
                                      onRestore,
}: SidebarItemProps) {
  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter' || e.key === ' ') {
      e.preventDefault()
      onSelect()
    }
  }

  return (
    <div
      role="button"
      tabIndex={0}
      className={`${styles.item} ${isSelected ? styles.active : ''}`}
      onClick={onSelect}
      onKeyDown={handleKeyDown}
    >
      <div className={styles.avatar} style={{ background: avatarColor(project.title) }}>
        {project.title[0]?.toUpperCase()}
      </div>
      <div className={styles.itemInfo}>
        <span className={styles.itemTitle}>{project.title}</span>
        <span className={styles.itemDate}>{formatUpdatedAt(project.updated_at)}</span>
      </div>
      {isArchived && onRestore && (
          <button
              className={styles.itemAction}
              onClick={onRestore}
              title="Восстановить проект"
              tabIndex={0}
          >
            <RestoreIcon size={14}/>
          </button>
      )}
    </div>
  )
}
