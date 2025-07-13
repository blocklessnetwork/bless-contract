import { AppProviders } from "@/components/app-providers.tsx";
import { AppLayout } from "@/components/app-layout.tsx";
import { RouteObject, useRoutes } from "react-router";
import { lazy } from "react";

const links = [
  //
  { label: "Home", path: "/" },
  { label: "Account", path: "/account" },
  { label: "BlessTime", path: "/blesstime" },
];

const LazyAccountIndex = lazy(
  () => import("@/components/account/account-index-feature"),
);
const LazyAccountDetail = lazy(
  () => import("@/components/account/account-detail-feature"),
);
const LazyDashboard = lazy(
  () => import("@/components/dashboard/dashboard-feature"),
);
const LazyRegistration = lazy(
  () => import("@/components/bless-time/bless-time-feature"),
);

const routes: RouteObject[] = [
  { index: true, element: <LazyDashboard /> },
  {
    path: "account",
    children: [
      { index: true, element: <LazyAccountIndex /> },
      { path: ":address", element: <LazyAccountDetail /> },
    ],
  },
  {
    path: "BlessTime",
    index: true,
    element: <LazyRegistration />,
  },
];

console.log({ links, routes });

export function App() {
  const router = useRoutes(routes);
  return (
    <AppProviders>
      <AppLayout links={links}>{router}</AppLayout>
    </AppProviders>
  );
}
