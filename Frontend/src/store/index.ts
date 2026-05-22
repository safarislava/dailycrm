import { configureStore } from '@reduxjs/toolkit'
import { crmApi } from './crmApi'
import uiReducer from './uiSlice'

export const store = configureStore({
  reducer: {
    [crmApi.reducerPath]: crmApi.reducer,
    ui: uiReducer,
  },
  middleware: (getDefaultMiddleware) =>
    getDefaultMiddleware().concat(crmApi.middleware),
})

export type RootState = ReturnType<typeof store.getState>
export type AppDispatch = typeof store.dispatch