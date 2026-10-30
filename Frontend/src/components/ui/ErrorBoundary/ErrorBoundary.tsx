import {Component, type ErrorInfo, type ReactNode} from 'react'
import styles from './ErrorBoundary.module.scss'

interface Props {
  children: ReactNode
  fallbackTitle?: string
}

interface State {
  hasError: boolean
  error: Error | null
}

export default class ErrorBoundary extends Component<Props, State> {
  public state: State = {
    hasError: false,
    error: null,
  }

  public static getDerivedStateFromError(error: Error): State {
    return { hasError: true, error }
  }

  public componentDidCatch(error: Error, errorInfo: ErrorInfo) {
    console.error('Unhandled React Error:', error, errorInfo)
  }

  private handleReset = () => {
    this.setState({ hasError: false, error: null })
  }

  public render() {
    const error = this.state.error
    if (this.state.hasError) {
      return (
        <div className={styles.container}>
          <h3 className={styles.title}>{this.props.fallbackTitle ?? 'Что-то пошло не так'}</h3>
          <p className={styles.description}>
            {error && error.message ? error.message : 'Произошла непредвиденная ошибка интерфейса. Попробуйте обновить этот блок.'}
          </p>
          <button className={styles.retryBtn} onClick={this.handleReset}>
            Попробовать снова
          </button>
        </div>
      )
    }

    return this.props.children
  }
}
