import React, { useState, useEffect } from 'react'
import { useDispatch } from 'react-redux'
import type { AppDispatch } from '../../store'
import { setAccessToken } from '../../store/authSlice'
import { useLoginMutation, useRegisterMutation } from '../../store/crmApi'
import styles from './LoginPage.module.scss'

export default function LoginPage() {
  const dispatch = useDispatch<AppDispatch>()
  const [mode, setMode] = useState<'login' | 'register'>('login')
  const [username, setUsername] = useState('')
  const [password, setPassword] = useState('')
  const [email, setEmail] = useState('')
  const [inviteToken, setInviteToken] = useState('')
  const [error, setError] = useState<string | null>(null)

  const [login, { isLoading: loggingIn }] = useLoginMutation()
  const [register, { isLoading: registering }] = useRegisterMutation()

  useEffect(() => {
    const params = new URLSearchParams(window.location.search)
    const token = params.get('invite')
    if (token) {
      setInviteToken(token)
      setMode('register')
      window.history.replaceState({}, '', window.location.pathname)
    }
  }, [])

  const loading = loggingIn || registering

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault()
    setError(null)

    if (mode === 'register') {
      const result = await register({
        username: username.trim(),
        password,
        email: email.trim(),
        invite_token: inviteToken.trim(),
      })
      if ('error' in result) {
        const status = (result.error as { status?: number })?.status
        if (status === 409) setError('Пользователь уже существует')
        else if (status === 403) setError('Недействительный или просроченный инвайт')
        else setError('Что-то пошло не так')
        return
      }
      setMode('login')
      setPassword('')
      setEmail('')
      setInviteToken('')
      return
    }

    const result = await login({ username: username.trim(), password })
    if ('data' in result && result.data) {
      dispatch(setAccessToken(result.data.access_token))
    } else {
      const status = (result.error as { status?: number })?.status
      if (status === 401) setError('Неверный логин или пароль')
      else setError('Что-то пошло не так')
    }
  }

  return (
    <div className={styles.page}>
      <form className={styles.card} onSubmit={handleSubmit}>
        <h1 className={styles.title}>CRM</h1>
        <div className={styles.tabs}>
          <button
            type="button"
            className={`${styles.tab} ${mode === 'login' ? styles.tabActive : ''}`}
            onClick={() => { setMode('login'); setError(null) }}
          >
            Войти
          </button>
          <button
            type="button"
            className={`${styles.tab} ${mode === 'register' ? styles.tabActive : ''}`}
            onClick={() => { setMode('register'); setError(null) }}
          >
            Регистрация
          </button>
        </div>
        <input
          className={styles.input}
          placeholder="Имя пользователя"
          value={username}
          onChange={(e) => setUsername(e.target.value)}
          autoComplete="username"
          required
        />
        <input
          className={styles.input}
          type="password"
          placeholder="Пароль"
          value={password}
          onChange={(e) => setPassword(e.target.value)}
          autoComplete={mode === 'login' ? 'current-password' : 'new-password'}
          required
        />
        {mode === 'register' && (
          <>
            <input
              className={styles.input}
              type="email"
              placeholder="Email"
              value={email}
              onChange={(e) => setEmail(e.target.value)}
              autoComplete="email"
              required
            />
            <input
              className={styles.input}
              placeholder="Инвайт-токен"
              value={inviteToken}
              onChange={(e) => setInviteToken(e.target.value)}
              autoComplete="off"
              required
            />
          </>
        )}
        {error && <p className={styles.error}>{error}</p>}
        <button className={styles.submit} type="submit" disabled={loading}>
          {loading ? '…' : mode === 'login' ? 'Войти' : 'Создать аккаунт'}
        </button>
        <p className={styles.support}>
          Техподдержка:{' '}
          <a href="mailto:safarislava@gmail.com">safarislava@gmail.com</a>
        </p>
      </form>
    </div>
  )
}
