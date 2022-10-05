import Header from "./components/header.tsx";
import { Route, Routes } from "react-router-dom";
import Home from "./pages/home.tsx";
import BookService from "./pages/bookService.tsx";

function App() {
  return (
    <div>
      <Header />
      <div>
        <Routes>
          <Route path="/" element={<Home />} />
          <Route path="/services/:id" element={<BookService />} />
        </Routes>
      </div>
    </div>
  );
}

export default App;
