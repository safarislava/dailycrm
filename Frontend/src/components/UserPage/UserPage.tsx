import React, { useState } from 'react'
import { useDispatch, useSelector } from 'react-redux'
import type { AppDispatch, RootState } from '../../store'
import { setUserPageOpen, setTheme, type Theme } from '../../store/uiSlice'
import {
  useGetMeQuery,
  useUpdateUsernameMutation,
  useUpdatePasswordMutation,
  useUpdateEmailMutation,
  useUpdateNotificationsMutation,
  useUpdateRolesMutation,
  useLogoutApiMutation,
  useCreateInviteMutation,
} from '../../store/crmApi'
import type { Role } from '../../types'
import FormModal from '../FormModal/FormModal'
import styles from './UserPage.module.scss'

type ModalKind = 'username' | 'email' | 'password'

export default function UserPage() {
  const dispatch = useDispatch<AppDispatch>()
  const { data: me } = useGetMeQuery()
  const theme = useSelector((s: RootState) => s.ui.theme)

  const [modal, setModal] = useState<ModalKind | null>(null)

  const [username, setUsername] = useState('')
  const [usernameError, setUsernameError] = useState<string | null>(null)

  const [email, setEmail] = useState('')
  const [emailError, setEmailError] = useState<string | null>(null)

  const [currentPassword, setCurrentPassword] = useState('')
  const [newPassword, setNewPassword] = useState('')
  const [confirmPassword, setConfirmPassword] = useState('')
  const [passwordError, setPasswordError] = useState<string | null>(null)

  const [rolesSuccess, setRolesSuccess] = useState(false)
  const [inviteToken, setInviteToken] = useState<string | null>(null)
  const [copied, setCopied] = useState(false)

  const [updateUsername, { isLoading: savingUsername }] = useUpdateUsernameMutation()
  const [updatePassword, { isLoading: savingPassword }] = useUpdatePasswordMutation()
  const [updateEmail, { isLoading: savingEmail }] = useUpdateEmailMutation()
  const [updateNotifications, { isLoading: savingNotifications }] = useUpdateNotificationsMutation()
  const [updateRoles, { isLoading: savingRoles }] = useUpdateRolesMutation()
  const [logoutApi] = useLogoutApiMutation()
  const [createInvite, { isLoading: creatingInvite }] = useCreateInviteMutation()

  const handleGenerateInvite = async () => {
    setInviteToken(null)
    setCopied(false)
    const result = await createInvite()
    if ('data' in result && result.data) setInviteToken(result.data.token)
  }

  const handleCopy = () => {
    if (!inviteToken) return
    navigator.clipboard.writeText(inviteToken)
    setCopied(true)
    setTimeout(() => setCopied(false), 2000)
  }

  const closeModal = () => {
    setModal(null)
    setUsernameError(null)
    setEmailError(null)
    setPasswordError(null)
  }

  const handleRoleToggle = async (role: Role) => {
    if (!me) return
    setRolesSuccess(false)
    const current = me.roles ?? []
    const next = current.includes(role) ? current.filter(r => r !== role) : [...current, role]
    const result = await updateRoles({ roles: next })
    if (!('error' in result)) setRolesSuccess(true)
  }

  const handleUsernameSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    setUsernameError(null)
    const result = await updateUsername({ username: username.trim() })
    if ('error' in result) {
      const status = (result.error as { status?: number })?.status
      setUsernameError(status === 409 ? 'Имя пользователя уже занято' : 'Что-то пошло не так')
    } else {
      setUsername('')
      closeModal()
    }
  }

  const handleEmailSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    setEmailError(null)
    const result = await updateEmail({ email: email.trim() })
    if ('error' in result) {
      const status = (result.error as { status?: number })?.status
      setEmailError(status === 409 ? 'Этот email уже используется' : 'Что-то пошло не так')
    } else {
      setEmail('')
      closeModal()
    }
  }

  const handlePasswordSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    setPasswordError(null)
    if (newPassword !== confirmPassword) {
      setPasswordError('Пароли не совпадают')
      return
    }
    const result = await updatePassword({ current_password: currentPassword, new_password: newPassword })
    if ('error' in result) {
      const status = (result.error as { status?: number })?.status
      setPasswordError(status === 401 ? 'Неверный текущий пароль' : 'Что-то пошло не так')
    } else {
      setCurrentPassword('')
      setNewPassword('')
      setConfirmPassword('')
      closeModal()
    }
  }

  return (
    <div className={styles.page}>
      {modal === 'username' && (
        <FormModal
          heading="Изменить имя пользователя"
          onClose={closeModal}
          onSubmit={handleUsernameSubmit}
          loading={savingUsername}
          error={usernameError}
          submitLabel="Сохранить"
        >
          <input
            className={styles.modalInput}
            placeholder="Новое имя пользователя"
            value={username}
            onChange={(e) => { setUsername(e.target.value); setUsernameError(null) }}
            autoComplete="username"
            autoFocus
            required
          />
        </FormModal>
      )}

      {modal === 'email' && (
        <FormModal
          heading="Изменить email"
          onClose={closeModal}
          onSubmit={handleEmailSubmit}
          loading={savingEmail}
          error={emailError}
          submitLabel="Сохранить"
        >
          <input
            className={styles.modalInput}
            type="email"
            placeholder="Новый email"
            value={email}
            onChange={(e) => { setEmail(e.target.value); setEmailError(null) }}
            autoComplete="email"
            autoFocus
            required
          />
        </FormModal>
      )}

      {modal === 'password' && (
        <FormModal
          heading="Изменить пароль"
          onClose={closeModal}
          onSubmit={handlePasswordSubmit}
          loading={savingPassword}
          error={passwordError}
          submitLabel="Изменить"
        >
          <input
            className={styles.modalInput}
            type="password"
            placeholder="Текущий пароль"
            value={currentPassword}
            onChange={(e) => { setCurrentPassword(e.target.value); setPasswordError(null) }}
            autoComplete="current-password"
            autoFocus
            required
          />
          <input
            className={styles.modalInput}
            type="password"
            placeholder="Новый пароль"
            value={newPassword}
            onChange={(e) => { setNewPassword(e.target.value); setPasswordError(null) }}
            autoComplete="new-password"
            required
          />
          <input
            className={styles.modalInput}
            type="password"
            placeholder="Повторите новый пароль"
            value={confirmPassword}
            onChange={(e) => { setConfirmPassword(e.target.value); setPasswordError(null) }}
            autoComplete="new-password"
            required
          />
        </FormModal>
      )}

      <header className={styles.header}>
        <button className={styles.back} onClick={() => dispatch(setUserPageOpen(false))}>
          <BackIcon />
        </button>
        <h1 className={styles.title}>Профиль</h1>
      </header>

      <div className={styles.content}>
        <section className={styles.section}>
          <div className={styles.sectionHeader}>
            <h2 className={styles.sectionTitle}>Имя пользователя</h2>
            <button className={styles.editBtn} onClick={() => setModal('username')}>Изменить</button>
          </div>
          {me && <p className={styles.current}>Текущее: <strong>{me.username}</strong></p>}
        </section>

        <div className={styles.divider} />

        <section className={styles.section}>
          <div className={styles.sectionHeader}>
            <h2 className={styles.sectionTitle}>Email</h2>
            <button className={styles.editBtn} onClick={() => setModal('email')}>Изменить</button>
          </div>
          {me && <p className={styles.current}>Текущий: <strong>{me.email}</strong></p>}
        </section>

        <div className={styles.divider} />

        <section className={styles.section}>
          <h2 className={styles.sectionTitle}>Роли</h2>
          <p className={styles.current}>Выберите одну или несколько ролей</p>
          {me && (
            <div className={styles.roles}>
              {(['gip', 'lawyer', 'accountant'] as Role[]).map(role => {
                const labels: Record<Role, string> = {
                  gip: 'ГИП',
                  lawyer: 'Юрист',
                  accountant: 'Бухгалтер',
                }
                const active = (me.roles ?? []).includes(role)
                return (
                  <button
                    key={role}
                    className={`${styles.roleBtn} ${active ? styles.roleBtnActive : ''}`}
                    onClick={() => handleRoleToggle(role)}
                    disabled={savingRoles}
                  >
                    {labels[role]}
                  </button>
                )
              })}
            </div>
          )}
          {rolesSuccess && <p className={styles.success}>Роли обновлены</p>}
        </section>

        <div className={styles.divider} />

        <section className={styles.section}>
          <h2 className={styles.sectionTitle}>Уведомления</h2>
          <p className={styles.current}>Письма о дедлайнах на следующий день</p>
          {me && (
            <button
              className={`${styles.btn} ${me.notifications_enabled ? styles.btnDanger : ''}`}
              onClick={() => updateNotifications({ enabled: !me.notifications_enabled })}
              disabled={savingNotifications}
            >
              {savingNotifications ? '…' : me.notifications_enabled ? 'Отключить' : 'Включить'}
            </button>
          )}
        </section>

        <div className={styles.divider} />

        <section className={styles.section}>
          <h2 className={styles.sectionTitle}>Тема</h2>
          <div className={styles.themeSwitch}>
            {(['dark', 'auto', 'light'] as Theme[]).map((t) => (
              <button
                key={t}
                className={`${styles.themeBtn} ${theme === t ? styles.themeBtnActive : ''}`}
                onClick={() => dispatch(setTheme(t))}
              >
                {t === 'dark' && <MoonIcon />}
                {t === 'auto' && <MonitorIcon />}
                {t === 'light' && <SunIcon />}
                {t === 'dark' ? 'Тёмная' : t === 'auto' ? 'Авто' : 'Светлая'}
              </button>
            ))}
          </div>
        </section>

        <div className={styles.divider} />

        <section className={styles.section}>
          <div className={styles.sectionHeader}>
            <h2 className={styles.sectionTitle}>Пароль</h2>
            <button className={styles.editBtn} onClick={() => setModal('password')}>Изменить</button>
          </div>
        </section>

        <div className={styles.divider} />

        <section className={styles.section}>
          <h2 className={styles.sectionTitle}>Приглашение</h2>
          <p className={styles.current}>Токен действует 7 дней</p>
          <button
            className={styles.btn}
            onClick={handleGenerateInvite}
            disabled={creatingInvite}
          >
            {creatingInvite ? '…' : 'Создать приглашение'}
          </button>
          {inviteToken && (
            <div className={styles.tokenBox}>
              <span className={styles.tokenText}>{inviteToken}</span>
              <button className={styles.copyBtn} onClick={handleCopy}>
                {copied ? <CheckIcon /> : <CopyIcon />}
              </button>
            </div>
          )}
        </section>

        <div className={styles.divider} />

        <section className={styles.section}>
          <button className={`${styles.btn} ${styles.btnDanger}`} onClick={() => logoutApi()}>
            Выйти из аккаунта
          </button>
        </section>
      </div>
    </div>
  )
}

function BackIcon() {
  return (
    <svg width="20" height="20" viewBox="0 0 24 24" fill="none">
      <path d="M19 12H5M12 5l-7 7 7 7" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"/>
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

function CheckIcon() {
  return (
    <svg width="14" height="14" viewBox="0 0 24 24" fill="none">
      <path d="M20 6 9 17l-5-5" stroke="currentColor" strokeWidth="2.5" strokeLinecap="round" strokeLinejoin="round"/>
    </svg>
  )
}

function MoonIcon() {
  return (
    <svg width="13" height="13" viewBox="0 0 24 24" fill="none">
      <path d="M21 12.79A9 9 0 1 1 11.21 3a7 7 0 0 0 9.79 9.79Z"
        stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"/>
    </svg>
  )
}

function SunIcon() {
  return (
    <svg width="13" height="13" viewBox="0 0 24 24" fill="none">
      <circle cx="12" cy="12" r="5" stroke="currentColor" strokeWidth="2"/>
      <line x1="12" y1="2" x2="12" y2="4" stroke="currentColor" strokeWidth="2" strokeLinecap="round"/>
      <line x1="12" y1="20" x2="12" y2="22" stroke="currentColor" strokeWidth="2" strokeLinecap="round"/>
      <line x1="4.22" y1="4.22" x2="5.64" y2="5.64" stroke="currentColor" strokeWidth="2" strokeLinecap="round"/>
      <line x1="18.36" y1="18.36" x2="19.78" y2="19.78" stroke="currentColor" strokeWidth="2" strokeLinecap="round"/>
      <line x1="2" y1="12" x2="4" y2="12" stroke="currentColor" strokeWidth="2" strokeLinecap="round"/>
      <line x1="20" y1="12" x2="22" y2="12" stroke="currentColor" strokeWidth="2" strokeLinecap="round"/>
      <line x1="4.22" y1="19.78" x2="5.64" y2="18.36" stroke="currentColor" strokeWidth="2" strokeLinecap="round"/>
      <line x1="18.36" y1="5.64" x2="19.78" y2="4.22" stroke="currentColor" strokeWidth="2" strokeLinecap="round"/>
    </svg>
  )
}

function MonitorIcon() {
  return (
    <svg width="13" height="13" viewBox="0 0 24 24" fill="none">
      <rect x="2" y="3" width="20" height="14" rx="2" stroke="currentColor" strokeWidth="2"/>
      <line x1="8" y1="21" x2="16" y2="21" stroke="currentColor" strokeWidth="2" strokeLinecap="round"/>
      <line x1="12" y1="17" x2="12" y2="21" stroke="currentColor" strokeWidth="2" strokeLinecap="round"/>
    </svg>
  )
}