interface Props {
  children: React.ReactNode;
}

export default function Layout({ children }: Props) {
  return (
    <div
      className="mb-0 pb-0"
      id="layout"
      style={{
        height: "100vh",
        overflowY: "hidden",
        marginBottom: 0,
      }}
    >
      <main className="  pt-4 px-4 bg-[rgba(241,246,251,255)]  overflow-y-scroll">
        {children}
      </main>
      {/* <Aside /> */}
    </div>
  );
}
