import { createSlice, type PayloadAction } from '@reduxjs/toolkit'

interface AuthState {
  accessToken: string | null
  initialized: boolean
}

const authSlice = createSlice({
  name: 'auth',
  initialState: {
    accessToken: null,
    initialized: false,
  } as AuthState,
  reducers: {
    setAccessToken(state, action: PayloadAction<string>) {
      state.accessToken = action.payload
      state.initialized = true
    },
    setInitialized(state) {
      state.initialized = true
    },
    logout(state) {
      state.accessToken = null
    },
  },
})

export const { setAccessToken, setInitialized, logout } = authSlice.actions
export default authSlice.reducer