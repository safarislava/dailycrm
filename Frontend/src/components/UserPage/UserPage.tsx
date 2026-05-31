import React, { useState } from 'react'
import { useDispatch } from 'react-redux'
import type { AppDispatch } from '../../store'
import { setUserPageOpen } from '../../store/uiSlice'
import {
  useGetMeQuery,
  useUpdateUsernameMutation,
  useUpdatePasswordMutation,
  useUpdateEmailMutation,
  useUpdateNotificationsMutation,
  useUpdateRolesMutation,
} from '../../store/crmApi'
import type { Role } from '../../types'
import styles from './UserPage.module.scss'

export default function UserPage() {
  const dispatch = useDispatch<AppDispatch>()
  const { data: me } = useGetMeQuery()

  const [username, setUsername] = useState('')
  const [usernameError, setUsernameError] = useState<string | null>(null)
  const [usernameSuccess, setUsernameSuccess] = useState(false)

  const [currentPassword, setCurrentPassword] = useState('')
  const [newPassword, setNewPassword] = useState('')
  const [confirmPassword, setConfirmPassword] = useState('')
  const [passwordError, setPasswordError] = useState<string | null>(null)
  const [passwordSuccess, setPasswordSuccess] = useState(false)

  const [email, setEmail] = useState('')
  const [emailError, setEmailError] = useState<string | null>(null)
  const [emailSuccess, setEmailSuccess] = useState(false)

  const [rolesSuccess, setRolesSuccess] = useState(false)

  const [updateUsername, { isLoading: savingUsername }] = useUpdateUsernameMutation()
  const [updatePassword, { isLoading: savingPassword }] = useUpdatePasswordMutation()
  const [updateEmail, { isLoading: savingEmail }] = useUpdateEmailMutation()
  const [updateNotifications, { isLoading: savingNotifications }] = useUpdateNotificationsMutation()
  const [updateRoles, { isLoading: savingRoles }] = useUpdateRolesMutation()

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
    setUsernameSuccess(false)
    const result = await updateUsername({ username: username.trim() })
    if ('error' in result) {
      const status = (result.error as { status?: number })?.status
      if (status === 409) setUsernameError('Имя пользователя уже занято')
      else setUsernameError('Что-то пошло не так')
    } else {
      setUsernameSuccess(true)
      setUsername('')
    }
  }

  const handleEmailSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    setEmailError(null)
    setEmailSuccess(false)
    const result = await updateEmail({ email: email.trim() })
    if ('error' in result) {
      const status = (result.error as { status?: number })?.status
      if (status === 409) setEmailError('Этот email уже используется')
      else setEmailError('Что-то пошло не так')
    } else {
      setEmailSuccess(true)
      setEmail('')
    }
  }

  const handlePasswordSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    setPasswordError(null)
    setPasswordSuccess(false)
    if (newPassword !== confirmPassword) {
      setPasswordError('Пароли не совпадают')
      return
    }
    const result = await updatePassword({ current_password: currentPassword, new_password: newPassword })
    if ('error' in result) {
      const status = (result.error as { status?: number })?.status
      if (status === 401) setPasswordError('Неверный текущий пароль')
      else setPasswordError('Что-то пошло не так')
    } else {
      setPasswordSuccess(true)
      setCurrentPassword('')
      setNewPassword('')
      setConfirmPassword('')
    }
  }

  return (
    <div className={styles.page}>
      <header className={styles.header}>
        <button className={styles.back} onClick={() => dispatch(setUserPageOpen(false))}>
          <BackIcon />
        </button>
        <h1 className={styles.title}>Профиль</h1>
      </header>

      <div className={styles.content}>
        <section className={styles.section}>
          <h2 className={styles.sectionTitle}>Имя пользователя</h2>
          {me && <p className={styles.current}>Текущее: <strong>{me.username}</strong></p>}
          <form className={styles.form} onSubmit={handleUsernameSubmit}>
            <input
              className={styles.input}
              placeholder="Новое имя пользователя"
              value={username}
              onChange={(e) => { setUsername(e.target.value); setUsernameSuccess(false) }}
              autoComplete="username"
              required
            />
            {usernameError && <p className={styles.error}>{usernameError}</p>}
            {usernameSuccess && <p className={styles.success}>Имя изменено</p>}
            <button className={styles.btn} type="submit" disabled={savingUsername || !username.trim()}>
              {savingUsername ? '…' : 'Сохранить'}
            </button>
          </form>
        </section>

        <div className={styles.divider} />

        <section className={styles.section}>
          <h2 className={styles.sectionTitle}>Email</h2>
          {me && <p className={styles.current}>Текущий: <strong>{me.email}</strong></p>}
          <form className={styles.form} onSubmit={handleEmailSubmit}>
            <input
              className={styles.input}
              type="email"
              placeholder="Новый email"
              value={email}
              onChange={(e) => { setEmail(e.target.value); setEmailSuccess(false) }}
              autoComplete="email"
              required
            />
            {emailError && <p className={styles.error}>{emailError}</p>}
            {emailSuccess && <p className={styles.success}>Email изменён</p>}
            <button className={styles.btn} type="submit" disabled={savingEmail || !email.trim()}>
              {savingEmail ? '…' : 'Сохранить'}
            </button>
          </form>
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
          <h2 className={styles.sectionTitle}>Пароль</h2>
          <form className={styles.form} onSubmit={handlePasswordSubmit}>
            <input
              className={styles.input}
              type="password"
              placeholder="Текущий пароль"
              value={currentPassword}
              onChange={(e) => { setCurrentPassword(e.target.value); setPasswordSuccess(false) }}
              autoComplete="current-password"
              required
            />
            <input
              className={styles.input}
              type="password"
              placeholder="Новый пароль"
              value={newPassword}
              onChange={(e) => { setNewPassword(e.target.value); setPasswordSuccess(false) }}
              autoComplete="new-password"
              required
            />
            <input
              className={styles.input}
              type="password"
              placeholder="Повторите новый пароль"
              value={confirmPassword}
              onChange={(e) => { setConfirmPassword(e.target.value); setPasswordSuccess(false) }}
              autoComplete="new-password"
              required
            />
            {passwordError && <p className={styles.error}>{passwordError}</p>}
            {passwordSuccess && <p className={styles.success}>Пароль изменён</p>}
            <button className={styles.btn} type="submit" disabled={savingPassword}>
              {savingPassword ? '…' : 'Изменить пароль'}
            </button>
          </form>
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