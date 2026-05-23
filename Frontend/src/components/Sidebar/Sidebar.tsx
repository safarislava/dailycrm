import React, { useState, useMemo, useEffect, useRef } from 'react'
import { useDispatch, useSelector } from 'react-redux'
import type { AppDispatch, RootState } from '../../store'
import { selectProject, selectStage, setUserPageOpen } from '../../store/uiSlice'
import {
  useGetProjectsQuery,
  useCreateProjectMutation,
  useDeleteProjectMutation,
  useGetDeadlinesQuery,
  useLogoutApiMutation,
  useCreateInviteMutation,
} from '../../store/crmApi'
import ConfirmDeleteModal from '../ConfirmDeleteModal/ConfirmDeleteModal'
import styles from './Sidebar.module.scss'

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

const AVATAR_COLORS = [
  '#e17076', '#7bc862', '#65aadd',
  '#a695e7', '#ee7aae', '#faa774', '#6ec9cb',
]

const avatarColor = (title: string) =>
  AVATAR_COLORS[title.charCodeAt(0) % AVATAR_COLORS.length]

function deadlineDiffDays(iso: string) {
  return Math.floor((new Date(iso).getTime() - Date.now()) / 86_400_000)
}

function formatDeadlineDate(iso: string): string {
  const d = new Date(iso)
  const diff = deadlineDiffDays(iso)
  if (diff < 0) return d.toLocaleDateString('ru-RU', { day: 'numeric', month: 'short' })
  if (diff === 0) return 'Сегодня'
  if (diff === 1) return 'Завтра'
  return d.toLocaleDateString('ru-RU', { day: 'numeric', month: 'short' })
}

function deadlineUrgency(iso: string): 'overdue' | 'urgent' | 'soon' | 'normal' {
  const diff = deadlineDiffDays(iso)
  if (diff < 0) return 'overdue'
  if (diff <= 1) return 'urgent'
  if (diff <= 7) return 'soon'
  return 'normal'
}

export default function Sidebar() {
  const dispatch = useDispatch<AppDispatch>()
  const selectedId = useSelector((s: RootState) => s.ui.selectedProjectId)

  const [search, setSearch] = useState('')
  const [composing, setComposing] = useState(false)
  const [newTitle, setNewTitle] = useState('')
  const [deadlinesOpen, setDeadlinesOpen] = useState(false)
  const [inviteToken, setInviteToken] = useState<string | null>(null)
  const bellRef = useRef<HTMLButtonElement>(null)
  const dropdownRef = useRef<HTMLDivElement>(null)
  const inviteRef = useRef<HTMLDivElement>(null)

  const { data: projects = [], isLoading } = useGetProjectsQuery()
  const [createProject, { isLoading: creating }] = useCreateProjectMutation()
  const [deleteProject] = useDeleteProjectMutation()
  const [logoutApi] = useLogoutApiMutation()
  const [createInvite, { isLoading: creatingInvite }] = useCreateInviteMutation()

  const [pendingDelete, setPendingDelete] = useState<{ id: string; title: string } | null>(null)

  const { data: allDeadlines = [] } = useGetDeadlinesQuery()

  const deadlineItems = useMemo(() => {
    const cutoff = Date.now() + 30 * 86_400_000
    return allDeadlines
      .filter((d) => !d.stage.completed && new Date(d.stage.deadline!).getTime() <= cutoff)
  }, [allDeadlines])

  const overdueCount = useMemo(
    () => deadlineItems.filter((d) => deadlineDiffDays(d.stage.deadline!) < 0).length,
    [deadlineItems],
  )

  useEffect(() => {
    if (!deadlinesOpen) return
    const handler = (e: MouseEvent) => {
      if (
        !bellRef.current?.contains(e.target as Node) &&
        !dropdownRef.current?.contains(e.target as Node)
      ) setDeadlinesOpen(false)
    }
    document.addEventListener('mousedown', handler)
    return () => document.removeEventListener('mousedown', handler)
  }, [deadlinesOpen])

  useEffect(() => {
    if (!inviteToken) return
    const handler = (e: MouseEvent) => {
      if (!inviteRef.current?.contains(e.target as Node)) setInviteToken(null)
    }
    document.addEventListener('mousedown', handler)
    return () => document.removeEventListener('mousedown', handler)
  }, [inviteToken])

  const handleGenerateInvite = async () => {
    const result = await createInvite()
    if ('data' in result && result.data) setInviteToken(result.data.token)
  }

  const filtered = useMemo(
    () => projects.filter((p) => p.title.toLowerCase().includes(search.toLowerCase())),
    [projects, search],
  )

  const submitCreate = async () => {
    const title = newTitle.trim()
    if (!title) return
    await createProject({ title })
    setNewTitle('')
    setComposing(false)
  }

  const handleCreateKey = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter') submitCreate()
    if (e.key === 'Escape') { setComposing(false); setNewTitle('') }
  }

  const handleDelete = (e: React.MouseEvent, id: string, title: string) => {
    e.stopPropagation()
    setPendingDelete({ id, title })
  }

  const confirmDelete = async () => {
    if (!pendingDelete) return
    await deleteProject(pendingDelete.id)
    if (selectedId === pendingDelete.id) dispatch(selectProject(null))
    setPendingDelete(null)
  }

  return (
    <aside className={styles.sidebar}>
      {pendingDelete && (
        <ConfirmDeleteModal
          heading="Удалить проект"
          name={pendingDelete.title}
          onConfirm={confirmDelete}
          onCancel={() => setPendingDelete(null)}
        />
      )}
      <header className={styles.header}>
        <span className={styles.logo}>CRM</span>
        <div className={styles.headerActions}>
          <button
            ref={bellRef}
            className={`${styles.bellBtn} ${deadlinesOpen ? styles.bellActive : ''}`}
            onClick={() => setDeadlinesOpen((v) => !v)}
            title="Ближайшие дедлайны"
          >
            <BellIcon />
            {overdueCount > 0 && (
              <span className={styles.badge}>{overdueCount > 9 ? '9+' : overdueCount}</span>
            )}
          </button>
          <button
            className={styles.composeBtn}
            onClick={handleGenerateInvite}
            disabled={creatingInvite}
            title="Пригласить пользователя"
          >
            <InviteIcon />
          </button>
          <button
            className={styles.composeBtn}
            onClick={() => dispatch(setUserPageOpen(true))}
            title="Профиль"
          >
            <ProfileIcon />
          </button>
          <button
            className={styles.composeBtn}
            onClick={() => logoutApi()}
            title="Выйти"
          >
            <LogoutIcon />
          </button>
        </div>
      </header>

      {inviteToken && (
        <div ref={inviteRef} className={styles.invitePopup}>
          <div className={styles.inviteLabel}>Инвайт-токен (действует 7 дней)</div>
          <div className={styles.inviteRow}>
            <span className={styles.inviteToken}>{inviteToken}</span>
            <button
              className={styles.inviteCopy}
              onClick={() => navigator.clipboard.writeText(inviteToken)}
              title="Скопировать"
            >
              <CopyIcon />
            </button>
          </div>
          <div className={styles.inviteHint}>Отправьте токен пользователю — он вводит его при регистрации</div>
        </div>
      )}

      {deadlinesOpen && (
        <div ref={dropdownRef} className={styles.deadlineDropdown}>
          <div className={styles.deadlineDropdownHeader}>Ближайшие дедлайны</div>
          {deadlineItems.length === 0 ? (
            <div className={styles.deadlineEmpty}>Нет предстоящих дедлайнов</div>
          ) : (
            deadlineItems.map((item) => (
              <button
                key={`${item.stage.project_id}-${item.stage.position}`}
                className={styles.deadlineItem}
                onClick={() => {
                  dispatch(selectProject(item.stage.project_id))
                  dispatch(selectStage(String(item.stage.position)))
                  setDeadlinesOpen(false)
                }}
              >
                <span
                  className={`${styles.deadlineDate} ${styles[`deadline_${deadlineUrgency(item.stage.deadline!)}`]}`}
                >
                  {formatDeadlineDate(item.stage.deadline!)}
                </span>
                <div className={styles.deadlineInfo}>
                  <span className={styles.deadlineProject}>{item.project_title}</span>
                  <span className={styles.deadlineStage}>{item.stage.title}</span>
                </div>
              </button>
            ))
          )}
        </div>
      )}


      <div className={styles.searchWrap}>
        <SearchIcon />
        <input
          className={styles.searchInput}
          placeholder="Поиск"
          value={search}
          onChange={(e) => setSearch(e.target.value)}
        />
        {search && (
          <button className={styles.clearBtn} onClick={() => setSearch('')}>
            <CloseIcon size={12} />
          </button>
        )}
      </div>

      <div className={styles.list}>
        {isLoading && (
          <div className={styles.hint}>Загрузка…</div>
        )}
        {!isLoading && filtered.length === 0 && (
          <div className={styles.hint}>
            {search ? 'Ничего не найдено' : 'Нет проектов'}
          </div>
        )}
        {filtered.map((project) => (
          <div
            key={project.id}
            className={`${styles.item} ${selectedId === project.id ? styles.active : ''}`}
            onClick={() => dispatch(selectProject(project.id))}
          >
            <div
              className={styles.avatar}
              style={{ background: avatarColor(project.title) }}
            >
              {project.title[0]?.toUpperCase()}
            </div>
            <div className={styles.itemInfo}>
              <span className={styles.itemTitle}>{project.title}</span>
              <span className={styles.itemDate}>{formatUpdatedAt(project.updated_at)}</span>
            </div>
            <button
              className={styles.itemDelete}
              onClick={(e) => handleDelete(e, project.id, project.title)}
              title="Удалить"
            >
              <CloseIcon size={11} />
            </button>
          </div>
        ))}
      </div>

      <div className={styles.footer}>
        {composing ? (
          <div className={styles.createRow}>
            <input
              className={styles.createInput}
              placeholder="Название проекта…"
              value={newTitle}
              onChange={(e) => setNewTitle(e.target.value)}
              onKeyDown={handleCreateKey}
              autoFocus
            />
            <button
              className={styles.createBtn}
              onClick={submitCreate}
              disabled={!newTitle.trim() || creating}
            >
              Создать
            </button>
            <button
              className={styles.createCancelBtn}
              onClick={() => { setComposing(false); setNewTitle('') }}
            >
              <CloseIcon size={14} />
            </button>
          </div>
        ) : (
          <button
            className={styles.newProjectBtn}
            onClick={() => setComposing(true)}
          >
            <ComposeIcon />
            Новый проект
          </button>
        )}
      </div>
    </aside>
  )
}

function SearchIcon() {
  return (
    <svg className={styles.searchIcon} width="16" height="16" viewBox="0 0 24 24" fill="none">
      <circle cx="11" cy="11" r="7" stroke="currentColor" strokeWidth="2"/>
      <path d="m16.5 16.5 4 4" stroke="currentColor" strokeWidth="2" strokeLinecap="round"/>
    </svg>
  )
}

function ComposeIcon() {
  return (
    <svg width="20" height="20" viewBox="0 0 24 24" fill="none">
      <path d="M12 5H7a2 2 0 0 0-2 2v10a2 2 0 0 0 2 2h10a2 2 0 0 0 2-2v-5"
        stroke="currentColor" strokeWidth="2" strokeLinecap="round"/>
      <path d="M18.5 2.5a2.121 2.121 0 0 1 3 3L13 14l-4 1 1-4 8.5-8.5Z"
        stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"/>
    </svg>
  )
}

function CloseIcon({ size = 14 }: { size?: number }) {
  return (
    <svg width={size} height={size} viewBox="0 0 24 24" fill="none">
      <path d="M18 6 6 18M6 6l12 12" stroke="currentColor" strokeWidth="2.5" strokeLinecap="round"/>
    </svg>
  )
}

function LogoutIcon() {
  return (
    <svg width="18" height="18" viewBox="0 0 24 24" fill="none">
      <path d="M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4" stroke="currentColor" strokeWidth="2" strokeLinecap="round"/>
      <polyline points="16 17 21 12 16 7" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"/>
      <line x1="21" y1="12" x2="9" y2="12" stroke="currentColor" strokeWidth="2" strokeLinecap="round"/>
    </svg>
  )
}

function BellIcon() {
  return (
    <svg width="18" height="18" viewBox="0 0 24 24" fill="none">
      <path d="M18 8A6 6 0 0 0 6 8c0 7-3 9-3 9h18s-3-2-3-9" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"/>
      <path d="M13.73 21a2 2 0 0 1-3.46 0" stroke="currentColor" strokeWidth="2" strokeLinecap="round"/>
    </svg>
  )
}

function ProfileIcon() {
  return (
    <svg width="18" height="18" viewBox="0 0 24 24" fill="none">
      <circle cx="12" cy="8" r="4" stroke="currentColor" strokeWidth="2"/>
      <path d="M4 20c0-4 3.6-7 8-7s8 3 8 7" stroke="currentColor" strokeWidth="2" strokeLinecap="round"/>
    </svg>
  )
}

function InviteIcon() {
  return (
    <svg width="18" height="18" viewBox="0 0 24 24" fill="none">
      <path d="M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"/>
      <circle cx="9" cy="7" r="4" stroke="currentColor" strokeWidth="2"/>
      <line x1="19" y1="8" x2="19" y2="14" stroke="currentColor" strokeWidth="2" strokeLinecap="round"/>
      <line x1="22" y1="11" x2="16" y2="11" stroke="currentColor" strokeWidth="2" strokeLinecap="round"/>
    </svg>
  )
}

function CopyIcon() {
  return (
    <svg width="14" height="14" viewBox="0 0 24 24" fill="none">
      <rect x="9" y="9" width="13" height="13" rx="2" stroke="currentColor" strokeWidth="2"/>
      <path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1" stroke="currentColor" strokeWidth="2" strokeLinecap="round"/>
    </svg>
  )
}