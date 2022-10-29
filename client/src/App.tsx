import Header from "./components/header";
import { Route, Routes } from "react-router-dom";
import Home from "./pages/home";
import Login from "./pages/login";
import Signup from "./pages/signup";
import BookService from "./pages/bookService";
import AdminOrders from "./pages/admin/orders";
import OrdersByService from "./pages/admin/orders/tableByServiceId";
import Contacts from "./pages/admin/contacts/table";
import AdminPanel from "./pages/adminPanel";
import AdminEditOrder from "./pages/admin/orders/edit";
import AdminEditContact from "./pages/admin/contacts/edit";
import EditContact from "./pages/admin/contacts/editSelf";

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
        <Route path="/services/:id" element={<BookService />} />
        <Route path="/admin/orders" element={<AdminOrders />} />
        <Route path="/admin/orders/:id" element={<OrdersByService />} />
        <Route path="/admin/orders/edit/:id" element={<AdminEditOrder />} />
        <Route path="/admin/contacts" element={<Contacts />} />
        <Route path="/admin/contacts/edit/:id" element={<AdminEditContact />} />
      </Routes>
    </>
  );
}

export default App;
