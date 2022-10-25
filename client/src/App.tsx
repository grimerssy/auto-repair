import Header from "./components/header";
import { Route, Routes } from "react-router-dom";
import Home from "./pages/home";
import Login from "./pages/login";
import BookService from "./pages/bookService";
import Orders from "./pages/orders";
import OrdersByService from "./pages/ordersByService";

function App() {
  return (
    <>
      <Header />
      <Routes>
        <Route path="/" element={<Home />} />
        <Route path="/auth/login" element={<Login />} />
        <Route path="/services/:id" element={<BookService />} />
        <Route path="/orders" element={<Orders />} />
        <Route path="/orders/:id" element={<OrdersByService />} />
      </Routes>
    </>
  );
}

export default App;
