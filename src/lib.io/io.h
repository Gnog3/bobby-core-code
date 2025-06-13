#ifndef LIB_IO_IO_H
#define LIB_IO_IO_H

#include <string_view>
#include <ios>
#include <limits>

#include <lib.hex/hex.h>

using namespace std::literals;

namespace io {
    const char endl = '\n';
    enum err {
        cerr
    };

    template <typename T>
    class Hex {
        static_assert(
            std::numeric_limits<T>::is_integer && 
            !std::numeric_limits<T>::is_signed && 
            std::numeric_limits<T>::digits <= 32
        );
      public:
        T _t;
        Hex(T _t) : _t(_t) {}
    };

    template <typename T>
    Hex<T> hex(T t) {
        return Hex(t);
    }

    const io::err operator<<(io::err err, char c);
    const io::err operator<<(io::err err, std::string_view s);

    // unsigned decimal
    template <typename T>
    std::enable_if_t<
        std::numeric_limits<T>::is_integer && 
        !std::numeric_limits<T>::is_signed && 
        std::numeric_limits<T>::digits <= 32, 
        const io::err // return type
    > operator<<(io::err err, const T t) {
        char s[17];
        __utoa(t, s, 10);
        io::cerr << std::string_view(s);
        return err;
    }

    // unsigned hexadecimal
    template <typename T>
    std::enable_if_t<
        std::numeric_limits<T>::is_integer && 
        !std::numeric_limits<T>::is_signed && 
        std::numeric_limits<T>::digits <= 32, 
        const io::err // return type
    > operator<<(io::err err, const Hex<T> t) {
        char s[8];
        int_to_hex_str(t._t, s);
        io::cerr << "0x" << std::string_view(s, 8);
        return err;
    }

    // signed decimal
    template <typename T>
    std::enable_if_t<
        std::numeric_limits<T>::is_integer && 
        std::numeric_limits<T>::is_signed && 
        std::numeric_limits<T>::digits <= 32, 
        const io::err // return type
    > operator<<(io::err err, const T t) {
        char s[17];
        __itoa(t, s, 10);
        io::cerr << s;
        return err;
    }
}

#endif
