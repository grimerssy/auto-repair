import Header from "./components/header";
import { Route, Routes } from "react-router-dom";
import Home from "./pages/home";
import Login from "./pages/login";
import Signup from "./pages/signup";
import BookService from "./pages/bookService";
import AdminOrders from "./pages/admin/orders";
import OrdersByService from "./pages/admin/orders/byServiceId";
import AdminPanel from "./pages/adminPanel";
import AdminServices from "./pages/admin/services";
import AdminEditOrder from "./pages/admin/orders/edit";
import EditContact from "./pages/admin/contacts/edit";

function App() {
  return (
    <>
      <Header />
      <Routes>
        <Route path="/" element={<Home />} />
        <Route path="/auth/login" element={<Login />} />
        <Route path="/auth/signup" element={<Signup />} />
        <Route path="/contacts/edit" element={<EditContact />} />
        <Route path="/admin" element={<AdminPanel />} />
        <Route path="/admin/services" element={<AdminServices />} />
        <Route path="/services/:id" element={<BookService />} />
        <Route path="/admin/orders" element={<AdminOrders />} />
        <Route path="/admin/orders/:id" element={<OrdersByService />} />
        <Route path="/admin/orders/edit/:id" element={<AdminEditOrder />} />
      </Routes>
    </>
  );
}

export default App;
