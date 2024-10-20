// fn add_tanstack_router {
//   // npm install @tanstack/react-router
//   // npm install -D @tanstack/router-plugin
//   // add plugin to vite config
//   // add routes folder in src, routes/__root.tsx, routes/index.tsx
//   // add routes folder in src,
// if untouched modify react template, else add .txt file with instructions
// OR use openAi to take app.tsx move wrappers into main.tsx and content into routes/index.tsx
//
// import { RouterProvider, createRouter } from '@tanstack/react-router'

// // Import the generated route tree
// import { routeTree } from './routeTree.gen'

// // Create a new router instance
// const router = createRouter({ routeTree })

// // Register the router instance for type safety
// declare module '@tanstack/react-router' {
//   interface Register {
//     router: typeof router
//   }
// }

// // Render the app
// const rootElement = document.getElementById('root')!
// if (!rootElement.innerHTML) {
//   const root = ReactDOM.createRoot(rootElement)
//   root.render(
//     <StrictMode>
//       <RouterProvider router={router} />
//     </StrictMode>,
//   )
// }
// }
