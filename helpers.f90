module helpers
  use, intrinsic :: iso_c_binding, only: c_associated, c_int, c_intptr_t, c_ptr
  implicit none

contains

  function data_size(data_start, data_end) bind(c)
    integer(c_int) :: data_size
    type(c_ptr), value :: data_start, data_end
    integer(c_intptr_t) :: start, end

    start = transfer(data_start, start)
    end = transfer(data_end, end)
    data_size = end - start
  end function data_size

  function is_null(cptr) bind(c)
    logical :: is_null
    type(c_ptr), value :: cptr

    is_null = .NOT. c_associated(cptr)
  end function is_null

end module helpers
