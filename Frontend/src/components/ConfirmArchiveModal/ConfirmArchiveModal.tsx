import {useEffect} from 'react'
import {createPortal} from 'react-dom'
import styles from './ConfirmArchiveModal.module.scss'

interface Props {
    heading?: string
    name: string
    confirmText?: string
    description?: string
    onConfirm: () => void
    onCancel: () => void
}

export default function ConfirmArchiveModal({
                                                heading = 'Архивировать проект',
                                                name,
                                                confirmText = 'В архив',
                                                description,
                                                onConfirm,
                                                onCancel,
                                            }: Props) {
    useEffect(() => {
        const handler = (e: KeyboardEvent) => {
            if (e.key === 'Escape') onCancel()
        }
        window.addEventListener('keydown', handler)
        return () => window.removeEventListener('keydown', handler)
    }, [onCancel])

    return createPortal(
        <div
            className={styles.overlay}
            onMouseDown={(e) => {
                if (e.target === e.currentTarget) onCancel()
            }}
        >
            <div className={styles.modal}>
                <h3 className={styles.heading}>{heading}</h3>
                <p className={styles.description}>
                    {description || (
                        <>
                            Проект <strong>{name}</strong> будет перемещен в архив. Вы сможете восстановить его в любой
                            момент.
                        </>
                    )}
                </p>
                <div className={styles.actions}>
                    <button className={styles.cancelBtn} onClick={onCancel}>
                        Отмена
                    </button>
                    <button className={styles.confirmBtn} onClick={onConfirm}>
                        {confirmText}
                    </button>
                </div>
            </div>
        </div>,
        document.body,
    )
}
