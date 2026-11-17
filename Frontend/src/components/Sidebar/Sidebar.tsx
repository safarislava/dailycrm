import React, {useEffect, useMemo, useRef, useState} from 'react'
import {useDispatch, useSelector} from 'react-redux'
import type {AppDispatch, RootState} from '../../store'
import {selectProject, setAdminPageOpen, setUserPageOpen, setViewArchive} from '../../store/uiSlice'
import {
    useCreateProjectMutation,
    useGetArchivedProjectsQuery,
    useGetDeadlinesQuery,
    useGetProjectsQuery,
    useRestoreProjectMutation,
} from '../../store/crmApi'
import {useGetMeQuery} from '../../store/api/usersApi'
import DeadlineDropdown, {deadlineDiffDays} from './components/DeadlineDropdown'
import SidebarItem from './components/SidebarItem'
import styles from './Sidebar.module.scss'

export default function Sidebar() {
  const dispatch = useDispatch<AppDispatch>()
  const selectedId = useSelector((s: RootState) => s.ui.selectedProjectId)
  const adminPageOpen = useSelector((s: RootState) => s.ui.adminPageOpen)
    const viewArchive = useSelector((s: RootState) => s.ui.viewArchive)

  const [search, setSearch] = useState('')
  const [composing, setComposing] = useState(false)
  const [newTitle, setNewTitle] = useState('')
  const [deadlinesOpen, setDeadlinesOpen] = useState(false)
  const bellRef = useRef<HTMLButtonElement>(null)
  const dropdownRef = useRef<HTMLDivElement>(null)

  const { data: me } = useGetMeQuery()
  const isAdmin = me?.roles?.includes('admin')

    const {data: projects = [], isLoading: loadingActive} = useGetProjectsQuery()
    const {data: archivedProjects = [], isLoading: loadingArchived} = useGetArchivedProjectsQuery(undefined, {
        skip: !viewArchive,
    })
  const [createProject, { isLoading: creating }] = useCreateProjectMutation()
    const [restoreProject] = useRestoreProjectMutation()

  const { data: allDeadlines = [] } = useGetDeadlinesQuery()



  const deadlineItems = useMemo(() => {
    const cutoff = Date.now() + 30 * 86_400_000
    return allDeadlines.filter(
        (d) =>
            d?.stage_id &&
            !d.stage_id.completed &&
            d.stage_id.deadline &&
            new Date(d.stage_id.deadline).getTime() <= cutoff,
    )
  }, [allDeadlines])

  const overdueCount = useMemo(
      () =>
          deadlineItems.filter((d) => d.stage_id?.deadline && deadlineDiffDays(d.stage_id.deadline) < 0)
              .length,
    [deadlineItems],
  )

  useEffect(() => {
    if (!deadlinesOpen) return
    const handler = (e: MouseEvent) => {
      if (
        !bellRef.current?.contains(e.target as Node) &&
        !dropdownRef.current?.contains(e.target as Node)
      )
        setDeadlinesOpen(false)
    }
    document.addEventListener('mousedown', handler)
    return () => document.removeEventListener('mousedown', handler)
  }, [deadlinesOpen])

    const currentList = useMemo(() => {
        const list = viewArchive ? archivedProjects : projects
        return list.filter((p) => p.title.toLowerCase().includes(search.toLowerCase()))
    }, [viewArchive, archivedProjects, projects, search])

    const isLoading = viewArchive ? loadingArchived : loadingActive

  const submitCreate = async () => {
    const title = newTitle.trim()
    if (!title) return
    await createProject({ title })
    setNewTitle('')
    setComposing(false)
  }

  const handleCreateKey = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter') submitCreate()
    if (e.key === 'Escape') {
      setComposing(false)
      setNewTitle('')
    }
  }

    const handleRestore = async (e: React.MouseEvent, id: string) => {
    e.stopPropagation()
        await restoreProject(id)
  }

  return (
    <aside className={styles.sidebar}>
      <header className={styles.header}>

      <span className={styles.logo}>DailyCRM</span>
        <div className={styles.headerActions}>
          {isAdmin && (
            <button
              className={`${styles.composeBtn} ${adminPageOpen ? styles.bellActive : ''}`}
              onClick={() => dispatch(setAdminPageOpen(true))}
              title="Админ-панель"
            >
              <ShieldIcon />
            </button>
          )}
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
            onClick={() => dispatch(setUserPageOpen(true))}
            title="Профиль"
          >
            <ProfileIcon />
          </button>
        </div>
      </header>

      {deadlinesOpen && (
        <DeadlineDropdown
          dropdownRef={dropdownRef}
          deadlineItems={deadlineItems}
          onClose={() => setDeadlinesOpen(false)}
          dispatch={dispatch}
        />
      )}

        <div className={styles.tabs}>
            <button
                className={`${styles.tab} ${!viewArchive ? styles.tabActive : ''}`}
                onClick={() => dispatch(setViewArchive(false))}
            >
                Активные
            </button>
            <button
                className={`${styles.tab} ${viewArchive ? styles.tabActive : ''}`}
                onClick={() => dispatch(setViewArchive(true))}
            >
                Архив
            </button>
        </div>

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
        {isLoading && <div className={styles.hint}>Загрузка…</div>}
          {!isLoading && currentList.length === 0 && (
          <div className={styles.hint}>
              {search
                  ? 'Ничего не найдено'
                  : viewArchive
                      ? 'В архиве пусто'
                      : 'Нет проектов'}
          </div>
        )}
          {currentList.map((project) => (
          <SidebarItem
            key={project.id}
            project={project}
            isSelected={selectedId === project.id}
            isArchived={viewArchive}
            onSelect={() => dispatch(selectProject(project.id))}
            onRestore={(e) => handleRestore(e, project.id)}
          />

          ))}
      </div>

        {!viewArchive && (
            <div className={styles.footer}>
                {composing ? (
                    <div className={styles.createRow}>
                        <input
                            className={styles.createInput}
                            placeholder="Название проекта…"
                            value={newTitle}
                            onChange={(e) => setNewTitle(e.target.value)}
                            onKeyDown={handleCreateKey}
                            onBlur={() => {
                                setComposing(false)
                                setNewTitle('')
                            }}
                            autoFocus
                        />
                        <button
                            className={styles.createBtn}
                            onMouseDown={(e) => e.preventDefault()}
                            onClick={submitCreate}
                            disabled={!newTitle.trim() || creating}
                        >
                            <SendIcon/>
                        </button>
                    </div>
                ) : (
                    <button className={styles.newProjectBtn} onClick={() => setComposing(true)}>
                        <ComposeIcon/>
                        Новый проект
                    </button>
                )}
            </div>
        )}

    </aside>
  )
}

function SearchIcon() {
  return (
    <svg className={styles.searchIcon} width="16" height="16" viewBox="0 0 24 24" fill="none">
      <circle cx="11" cy="11" r="7" stroke="currentColor" strokeWidth="2" />
      <path d="m16.5 16.5 4 4" stroke="currentColor" strokeWidth="2" strokeLinecap="round" />
    </svg>
  )
}

function ComposeIcon() {
  return (
    <svg width="20" height="20" viewBox="0 0 24 24" fill="none">
      <path
        d="M12 5H7a2 2 0 0 0-2 2v10a2 2 0 0 0 2 2h10a2 2 0 0 0 2-2v-5"
        stroke="currentColor"
        strokeWidth="2"
        strokeLinecap="round"
      />
      <path
        d="M18.5 2.5a2.121 2.121 0 0 1 3 3L13 14l-4 1 1-4 8.5-8.5Z"
        stroke="currentColor"
        strokeWidth="2"
        strokeLinecap="round"
        strokeLinejoin="round"
      />
    </svg>
  )
}

function CloseIcon({ size = 14 }: { size?: number }) {
  return (
    <svg width={size} height={size} viewBox="0 0 24 24" fill="none">
      <path d="M18 6 6 18M6 6l12 12" stroke="currentColor" strokeWidth="2.5" strokeLinecap="round" />
    </svg>
  )
}

function BellIcon() {
  return (
    <svg width="18" height="18" viewBox="0 0 24 24" fill="none">
      <path
        d="M18 8A6 6 0 0 0 6 8c0 7-3 9-3 9h18s-3-2-3-9"
        stroke="currentColor"
        strokeWidth="2"
        strokeLinecap="round"
        strokeLinejoin="round"
      />
      <path d="M13.73 21a2 2 0 0 1-3.46 0" stroke="currentColor" strokeWidth="2" strokeLinecap="round" />
    </svg>
  )
}

function ProfileIcon() {
  return (
    <svg width="18" height="18" viewBox="0 0 24 24" fill="none">
      <circle cx="12" cy="8" r="4" stroke="currentColor" strokeWidth="2" />
      <path d="M4 20c0-4 3.6-7 8-7s8 3 8 7" stroke="currentColor" strokeWidth="2" strokeLinecap="round" />
    </svg>
  )
}

function SendIcon() {
  return (
    <svg width="18" height="18" viewBox="0 0 24 24" fill="none">
      <line x1="22" y1="2" x2="11" y2="13" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" />
      <polygon
        points="22 2 15 22 11 13 2 9 22 2"
        stroke="currentColor"
        strokeWidth="2"
        strokeLinecap="round"
        strokeLinejoin="round"
        fill="none"
      />
    </svg>
  )
}

function ShieldIcon() {
  return (
    <svg width="18" height="18" viewBox="0 0 24 24" fill="none">
      <path
        d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"
        stroke="currentColor"
        strokeWidth="2"
        strokeLinecap="round"
        strokeLinejoin="round"
      />
    </svg>
  )
}

