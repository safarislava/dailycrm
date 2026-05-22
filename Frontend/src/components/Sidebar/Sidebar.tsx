import { useState, useMemo } from 'react'
import { useDispatch, useSelector } from 'react-redux'
import type { AppDispatch, RootState } from '../../store'
import { selectProject } from '../../store/uiSlice'
import {
  useGetProjectsQuery,
  useCreateProjectMutation,
  useDeleteProjectMutation,
} from '../../store/crmApi'
import styles from './Sidebar.module.scss'

const AVATAR_COLORS = [
  '#e17076', '#7bc862', '#65aadd',
  '#a695e7', '#ee7aae', '#faa774', '#6ec9cb',
]

const avatarColor = (title: string) =>
  AVATAR_COLORS[title.charCodeAt(0) % AVATAR_COLORS.length]

export default function Sidebar() {
  const dispatch = useDispatch<AppDispatch>()
  const selectedId = useSelector((s: RootState) => s.ui.selectedProjectId)

  const [search, setSearch] = useState('')
  const [composing, setComposing] = useState(false)
  const [newTitle, setNewTitle] = useState('')

  const { data: projects = [], isLoading } = useGetProjectsQuery()
  const [createProject, { isLoading: creating }] = useCreateProjectMutation()
  const [deleteProject] = useDeleteProjectMutation()

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

  const handleDelete = async (e: React.MouseEvent, id: string) => {
    e.stopPropagation()
    await deleteProject(id)
    if (selectedId === id) dispatch(selectProject(null))
  }

  return (
    <aside className={styles.sidebar}>
      <header className={styles.header}>
        <span className={styles.logo}>CRM</span>
        <button
          className={styles.composeBtn}
          onClick={() => { setComposing((v) => !v); setNewTitle('') }}
          title={composing ? 'Отмена' : 'Новый проект'}
        >
          {composing
            ? <CloseIcon />
            : <ComposeIcon />
          }
        </button>
      </header>

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

      {composing && (
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
        </div>
      )}

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
            <span className={styles.itemTitle}>{project.title}</span>
            <button
              className={styles.itemDelete}
              onClick={(e) => handleDelete(e, project.id)}
              title="Удалить"
            >
              <CloseIcon size={11} />
            </button>
          </div>
        ))}
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