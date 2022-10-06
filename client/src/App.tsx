import Header from "./components/header.tsx";
import { Route, Routes } from "react-router-dom";
import Home from "./pages/home.tsx";
import BookService from "./pages/bookService.tsx";
import Orders from "./pages/orders.tsx";
import OrdersByService from "./pages/ordersByService.tsx";

function App() {
  return (
    <div>
      <Header />
      <div>
        <Routes>
          <Route path="/" element={<Home />} />
          <Route path="/services/:id" element={<BookService />} />
          <Route path="/orders" element={<Orders />} />
          <Route path="/orders/:id" element={<OrdersByService />} />
        </Routes>
      </div>
    </div>
  );
}

export default App;
