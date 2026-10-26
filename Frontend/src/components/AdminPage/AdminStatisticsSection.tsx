import React from 'react'
import {useGetAdminStatisticsQuery} from '../../store/api/adminApi'
import {formatNumber} from '../MainPanel/components/Helpers'
import styles from './AdminPage.module.scss'

export default function AdminStatisticsSection() {
  const { data: stats, isLoading, error } = useGetAdminStatisticsQuery()

  if (isLoading) return <div className={styles.subtitle}>Загрузка статистики…</div>
  if (error || !stats) return <div className={styles.subtitle}>Не удалось загрузить статистику</div>

  return (
    <div className={styles.section}>
      <div className={styles.gridStats}>
        <div className={styles.statCard}>
          <div className={styles.statIcon}>
            <UsersIcon />
          </div>
          <div className={styles.statInfo}>
            <span className={styles.statValue}>{formatNumber(stats.total_users)}</span>
            <span className={styles.statLabel}>Пользователей</span>
          </div>
        </div>

        <div className={styles.statCard}>
          <div className={styles.statIcon}>
            <FolderIcon />
          </div>
          <div className={styles.statInfo}>
            <span className={styles.statValue}>{formatNumber(stats.total_projects)}</span>
            <span className={styles.statLabel}>Проектов</span>
          </div>
        </div>

        <div className={styles.statCard}>
          <div className={styles.statIcon}>
            <LayersIcon />
          </div>
          <div className={styles.statInfo}>
            <span className={styles.statValue}>{formatNumber(stats.total_stages)}</span>
            <span className={styles.statLabel}>Этапов</span>
          </div>
        </div>

        <div className={styles.statCard}>
          <div className={styles.statIcon}>
            <MailIcon />
          </div>
          <div className={styles.statInfo}>
            <span className={styles.statValue}>{formatNumber(stats.pending_invitations)}</span>
            <span className={styles.statLabel}>Активных приглашений</span>
          </div>
        </div>
      </div>
    </div>
  )
}

function UsersIcon() {
  return (
    <svg width="24" height="24" viewBox="0 0 24 24" fill="none">
      <path d="M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2" stroke="currentColor" strokeWidth="2" strokeLinecap="round" />
      <circle cx="9" cy="7" r="4" stroke="currentColor" strokeWidth="2" />
      <path d="M23 21v-2a4 4 0 0 0-3-3.87" stroke="currentColor" strokeWidth="2" strokeLinecap="round" />
      <path d="M16 3.13a4 4 0 0 1 0 7.75" stroke="currentColor" strokeWidth="2" strokeLinecap="round" />
    </svg>
  )
}

function FolderIcon() {
  return (
    <svg width="24" height="24" viewBox="0 0 24 24" fill="none">
      <path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" />
    </svg>
  )
}

function LayersIcon() {
  return (
    <svg width="24" height="24" viewBox="0 0 24 24" fill="none">
      <polygon points="12 2 2 7 12 12 22 7 12 2" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" />
      <polyline points="2 17 12 22 22 17" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" />
      <polyline points="2 12 12 17 22 12" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" />
    </svg>
  )
}

function MailIcon() {
  return (
    <svg width="24" height="24" viewBox="0 0 24 24" fill="none">
      <path d="M4 4h16c1.1 0 2 .9 2 2v12c0 1.1-.9 2-2 2H4c-1.1 0-2-.9-2-2V6c0-1.1.9-2 2-2z" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" />
      <polyline points="22,6 12,13 2,6" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" />
    </svg>
  )
}
