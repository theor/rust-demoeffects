use crate::bitmap::Bitmap;

// 30 x 80
const COCONUT_DATA: [u32;2400] = [
    0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFF00FF00,0xFF00FF00,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFF00FF00,0xFF00FF00,0xFF00FF00,0xFF00FF00,0xFF00FF00,0xFF00FF00,0xFF00FF00,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFF00FF00,0xFF00AA00,0xFF00AA00,0xFF00FF00,0xFF00AA00,0xFF00AA00,0xFF00FF00,0xFF00FF00,0xFF00FF00,0xFF00FF00,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFF00FF00,0xFF00FF00,0xFFFFFFFF,0xFFFFFFFF,0xFF00AA00,0xFF005500,0xFF00AA00,0xFF00FF00,0xFF00AA00,0xFF00FF00,0xFF00FF00,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFF00FF00,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFF005500,0xFF005500,0xFF00AA00,0xFF00AA00,0xFF00AA00,0xFF00FF00,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFF00FF00,0xFF00FF00,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFF00FF00,0xFF00FF00,0xFF00FF00,0xFF00FF00,0xFF00FF00,0xFF00FF00,0xFF00FF00,0xFFFFFFFF,0xFF005500,0xFF00AA00,0xFF00AA00,0xFF00FF00,0xFF00FF00,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFF00FF00,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFF00FF00,0xFF00AA00,0xFF00AA00,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFF00FF00,0xFF00AA00,0xFF00AA00,0xFF00AA00,0xFF00AA00,0xFF00AA00,0xFF00FF00,0xFF00FF00,0xFF00FF00,0xFF005500,0xFF005500,0xFF00AA00,0xFF00AA00,0xFF00FF00,0xFFFFFFFF,0xFFFFFFFF,0xFF00FF00,0xFF00FF00,0xFF00AA00,0xFFFFFFFF,0xFFFFFFFF,0xFF00FF00,0xFF00FF00,0xFF00AA00,0xFF005500,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFF00FF00,0xFF00FF00,0xFF00AA00,0xFF005500,0xFF005500,0xFF005500,0xFF005500,0xFF005500,0xFF00AA00,0xFF00AA00,0xFF00AA00,0xFF00FF00,0xFF005500,0xFF005500,0xFF00AA00,0xFF00FF00,0xFFFFFFFF,0xFF00FF00,0xFF00AA00,0xFF00AA00,0xFFFFFFFF,0xFF00FF00,0xFF00FF00,0xFF00AA00,0xFF00AA00,0xFF005500,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFF00FF00,0xFF00AA00,0xFF005500,0xFF00AA00,0xFF005500,0xFF00AA00,0xFF005500,0xFF00AA00,0xFF005500,0xFF005500,0xFF00AA00,0xFF00FF00,0xFF00FF00,0xFF005500,0xFF00AA00,0xFF005500,0xFF00AA00,0xFF00FF00,0xFF00AA00,0xFF005500,0xFF00FF00,0xFF00AA00,0xFF00AA00,0xFF005500,0xFF005500,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFF00FF00,0xFF00AA00,0xFF005500,0xFF00AA00,0xFF005500,0xFF00AA00,0xFF005500,0xFF00AA00,0xFF005500,0xFF005500,0xFF005500,0xFF00AA00,0xFF00FF00,0xFF005500,0xFF00AA00,0xFF005500,0xFF00AA00,0xFF00FF00,0xFF00FF00,0xFF00FF00,0xFF00AA00,0xFF00AA00,0xFF005500,0xFF005500,0xFF005500,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFF00AA00,0xFF005500,0xFF00AA00,0xFF00AA00,0xFF005500,0xFF00AA00,0xFF005500,0xFF00AA00,0xFFFFFFFF,0xFFFFFFFF,0xFF005500,0xFF005500,0xFF00AA00,0xFF00FF00,0xFF00FF00,0xFF00AA00,0xFF005500,0xFF005500,0xFF00FF00,0xFF00FF00,0xFF00FF00,0xFF00FF00,0xFF00FF00,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFF005500,0xFFFFFFFF,0xFF00AA00,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFF00FF00,0xFF00FF00,0xFFFFFFFF,0xFF005500,0xFF005500,0xFF00AA00,0xFF00AA00,0xFF00AA00,0xFF00FF00,0xFF00FF00,0xFF00AA00,0xFF00AA00,0xFF00AA00,0xFF00AA00,0xFF00AA00,0xFF00FF00,0xFF00FF00,0xFF00FF00,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFF005500,0xFFFFFFFF,0xFF00AA00,0xFFFFFFFF,0xFFFFFFFF,0xFF00FF00,0xFF00FF00,0xFF00FF00,0xFF00FF00,0xFF00FF00,0xFF00FF00,0xFF00FF00,0xFF00AA00,0xFF00AA00,0xFF00AA00,0xFF005500,0xFF00AA00,0xFF00AA00,0xFF005500,0xFF005500,0xFF005500,0xFF005500,0xFF005500,0xFF00AA00,0xFF00AA00,0xFF00AA00,0xFF00FF00,0xFF00FF00,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFF00FF00,0xFF00FF00,0xFF00AA00,0xFF00FF00,0xFF00AA00,0xFF00FF00,0xFF00FF00,0xFF00AA00,0xFF00AA00,0xFF005500,0xFF00AA00,0xFF00FF00,0xFF00FF00,0xFF005500,0xFF00AA00,0xFF00FF00,0xFF00FF00,0xFF005500,0xFF00FF00,0xFF00FF00,0xFFFFFFFF,0xFFFFFFFF,0xFF005500,0xFF00AA00,0xFF00AA00,0xFF00FF00,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFF00FF00,0xFF00FF00,0xFF00AA00,0xFF00FF00,0xFF00AA00,0xFF00FF00,0xFF00AA00,0xFF00AA00,0xFF00AA00,0xFF005500,0xFF00FF00,0xFF00AA00,0xFF00FF00,0xFF005500,0xFF00FF00,0xFF00AA00,0xFF00FF00,0xFF005500,0xFF00AA00,0xFF00AA00,0xFF00FF00,0xFF00FF00,0xFF005500,0xFF00AA00,0xFF00AA00,0xFF00FF00,0xFFFFFFFF,0xFFFFFFFF,0xFF00FF00,0xFF00FF00,0xFF00AA00,0xFF00AA00,0xFF00AA00,0xFF00AA00,0xFF005500,0xFF00AA00,0xFF00AA00,0xFF005500,0xFF00AA00,0xFF005500,0xFF00AA00,0xFF00AA00,0xFF00FF00,0xFF005500,0xFF005500,0xFF00AA00,0xFF00AA00,0xFF00FF00,0xFF00FF00,0xFF005500,0xFF00AA00,0xFF00AA00,0xFF00FF00,0xFF005500,0xFF005500,0xFF00AA00,0xFF00FF00,0xFFFFFFFF,0xFF00FF00,0xFF00AA00,0xFF00FF00,0xFF00AA00,0xFF005500,0xFF00AA00,0xFF005500,0xFF00AA00,0xFF00AA00,0xFF005500,0xFF00FF00,0xFF005500,0xFF00AA00,0xFF00AA00,0xFF00FF00,0xFF00FF00,0xFF005500,0xFF00AA00,0xFF00AA00,0xFF00FF00,0xFF00FF00,0xFF00AA00,0xFF005500,0xFF005500,0xFF00AA00,0xFF00FF00,0xFF005500,0xFF00AA00,0xFF00FF00,0xFFFFFFFF,0xFF00FF00,0xFF00AA00,0xFF00FF00,0xFF00AA00,0xFF005500,0xFF005500,0xFF00AA00,0xFF005500,0xFF005500,0xFF00FF00,0xFF00AA00,0xFF00AA00,0xFF005500,0xFF005500,0xFF00AA00,0xFF00FF00,0xFF00AA00,0xFF00FF00,0xFF005500,0xFF00AA00,0xFF00AA00,0xFF00FF00,0xFF00FF00,0xFF00FF00,0xFF005500,0xFF00AA00,0xFF00FF00,0xFF005500,0xFF00AA00,0xFF00FF00,0xFF00FF00,0xFF00AA00,0xFF00FF00,0xFF00AA00,0xFF005500,0xFF005500,0xFF00AA00,0xFF005500,0xFF00FF00,0xFF00FF00,0xFF00AA00,0xFF005500,0xFF005500,0xFF005500,0xFF00AA00,0xFF00FF00,0xFF005500,0xFF00FF00,0xFF00FF00,0xFF005500,0xFF00AA00,0xFF00FF00,0xFF00AA00,0xFF00FF00,0xFF00FF00,0xFF005500,0xFF00AA00,0xFF00FF00,0xFFFFFFFF,0xFF00FF00,0xFF00AA00,0xFF00AA00,0xFF00FF00,0xFF00AA00,0xFF005500,0xFF005500,0xFF005500,0xFF005500,0xFF00FF00,0xFF00FF00,0xFF00AA00,0xFF005500,0xFF005500,0xFF005500,0xFF00AA00,0xFF00FF00,0xFF005500,0xFF00AA00,0xFF00FF00,0xFF00FF00,0xFF00AA00,0xFF00AA00,0xFF00AA00,0xFF00AA00,0xFF00FF00,0xFF005500,0xFF00AA00,0xFF00FF00,0xFFFFFFFF,0xFF00FF00,0xFF00AA00,0xFF005500,0xFF00AA00,0xFF005500,0xFF005500,0xFFFFFFFF,0xFF005500,0xFF00FF00,0xFF00FF00,0xFF00AA00,0xFF005500,0xFF005500,0xFF005500,0xFF005500,0xFF00AA00,0xFF00FF00,0xFF005500,0xFF005500,0xFF00AA00,0xFF00FF00,0xFF005500,0xFF00AA00,0xFF00AA00,0xFF00FF00,0xFF00FF00,0xFFFFFFFF,0xFF005500,0xFF00AA00,0xFFFFFFFF,0xFF00AA00,0xFF00AA00,0xFF005500,0xFF00AA00,0xFF005500,0xFF005500,0xFFFFFFFF,0xFF005500,0xFF00FF00,0xFF00AA00,0xFF00AA00,0xFF005500,0xFF005500,0xFF005500,0xFF00AA00,0xFF00AA00,0xFF00FF00,0xFF00FF00,0xFF005500,0xFF005500,0xFF00AA00,0xFF00FF00,0xFF005500,0xFF00AA00,0xFF00AA00,0xFF00FF00,0xFFFFFFFF,0xFFFFFFFF,0xFF005500,0xFFFFFFFF,0xFF00AA00,0xFF005500,0xFF005500,0xFF005500,0xFF005500,0xFF005500,0xFFFFFFFF,0xFF00FF00,0xFF00AA00,0xFF00AA00,0xFF005500,0xFF005500,0xFF520000,0xFF005500,0xFF00AA00,0xFF005500,0xFF00AA00,0xFF00FF00,0xFF005500,0xFF005500,0xFF00AA00,0xFF00FF00,0xFFFFFFFF,0xFF005500,0xFF005500,0xFF00FF00,0xFF00AA00,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFF00FF00,0xFF00AA00,0xFF005500,0xFFFFFFFF,0xFFFFFFFF,0xFF005500,0xFFFFFFFF,0xFF00FF00,0xFF00AA00,0xFF00AA00,0xFF005500,0xFF005500,0xFF520000,0xFF005500,0xFF00AA00,0xFF005500,0xFF00AA00,0xFF00FF00,0xFF00AA00,0xFF005500,0xFF00AA00,0xFF00FF00,0xFFFFFFFF,0xFF005500,0xFF005500,0xFF00AA00,0xFF00FF00,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFF00FF00,0xFF00AA00,0xFF005500,0xFFFFFFFF,0xFFFFFFFF,0xFF005500,0xFFFFFFFF,0xFF00AA00,0xFF005500,0xFF00AA00,0xFF005500,0xFF005500,0xFFFFFFFF,0xFF005500,0xFF00AA00,0xFF005500,0xFF00AA00,0xFF00FF00,0xFF00AA00,0xFF005500,0xFF00AA00,0xFF00FF00,0xFFFFFFFF,0xFFFFFFFF,0xFF005500,0xFF00AA00,0xFF00FF00,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFF00FF00,0xFF00AA00,0xFF005500,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFF00FF00,0xFF00AA00,0xFF005500,0xFF00AA00,0xFF005500,0xFF005500,0xFFFFFFFF,0xFF005500,0xFF005500,0xFF005500,0xFF00AA00,0xFF00FF00,0xFF005500,0xFF005500,0xFF005500,0xFF00AA00,0xFF00FF00,0xFFFFFFFF,0xFF005500,0xFF00AA00,0xFF00FF00,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFF00FF00,0xFF00AA00,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFF00FF00,0xFF00FF00,0xFF00AA00,0xFF005500,0xFF005500,0xFF005500,0xFFFFFFFF,0xFF005500,0xFF00AA00,0xFF00AA00,0xFF00AA00,0xFF00FF00,0xFFFFFFFF,0xFFFFFFFF,0xFF005500,0xFF00AA00,0xFF00FF00,0xFFFFFFFF,0xFFFFFFFF,0xFF005500,0xFF00FF00,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFF00FF00,0xFF00AA00,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFF00AA00,0xFF00FF00,0xFF00AA00,0xFF005500,0xFF005500,0xFFFFFFFF,0xFFFFFFFF,0xFF00AA00,0xFF005500,0xFF00AA00,0xFF00FF00,0xFF00AA00,0xFFFFFFFF,0xFFFFFFFF,0xFF005500,0xFF00AA00,0xFF00FF00,0xFFFFFFFF,0xFFFFFFFF,0xFF005500,0xFF00AA00,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFF00AA00,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFF00AA00,0xFF00FF00,0xFF00AA00,0xFF00AA00,0xFF005500,0xFFFFFFFF,0xFFFFFFFF,0xFF00AA00,0xFF005500,0xFF00AA00,0xFF00FF00,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFF005500,0xFF00AA00,0xFF00FF00,0xFFFFFFFF,0xFFFFFFFF,0xFF00AA00,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFF00AA00,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFF520000,0xFF520000,0xFF00FF00,0xFF00AA00,0xFF005500,0xFFFFFFFF,0xFFFFFFFF,0xFF005500,0xFFFFFFFF,0xFF005500,0xFF00AA00,0xFF00FF00,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFF00AA00,0xFF00FF00,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFF00AA00,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFF520000,0xFF520000,0xFF00FF00,0xFF00AA00,0xFF005500,0xFFFFFFFF,0xFFFFFFFF,0xFF005500,0xFFFFFFFF,0xFF00AA00,0xFF00FF00,0xFF00FF00,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFF00FF00,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFF520000,0xFF520000,0xFFAC5500,0xFF00FF00,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFF005500,0xFF00AA00,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFF520000,0xFFACAAAC,0xFFACAAAC,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFF520000,0xFF520000,0xFFAC5500,0xFF520000,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFF520000,0xFFACAAAC,0xFFACAAAC,0xFF520000,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFF520000,0xFFAC5500,0xFF520000,0xFFACAAAC,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFF520000,0xFF520000,0xFFAC5500,0xFF520000,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFF520000,0xFFAC5500,0xFF520000,0xFFAC5500,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFF520000,0xFFAC5500,0xFF520000,0xFFACAAAC,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFF520000,0xFF520000,0xFFAC5500,0xFF520000,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFF520000,0xFFACAAAC,0xFFAC5500,0xFFACAAAC,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFF520000,0xFFAC5500,0xFF520000,0xFFAC5500,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFF520000,0xFF520000,0xFFAC5500,0xFFACAAAC,0xFFAC5500,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFF520000,0xFF520000,0xFFACAAAC,0xFFACAAAC,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFF520000,0xFF520000,0xFFAC5500,0xFF520000,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFF520000,0xFF520000,0xFFACAAAC,0xFFAC5500,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFACAAAC,0xFF520000,0xFF520000,0xFF520000,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFF520000,0xFFACAAAC,0xFF520000,0xFF520000,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFF520000,0xFF520000,0xFFAC5500,0xFF520000,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFF520000,0xFFAC5500,0xFFAC5500,0xFFACAAAC,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFACAAAC,0xFFACAAAC,0xFF520000,0xFF520000,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFF520000,0xFFAC5500,0xFFACAAAC,0xFFAC5500,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFF520000,0xFF520000,0xFF520000,0xFFACAAAC,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFF520000,0xFFAC5500,0xFFACAAAC,0xFFAC5500,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFACAAAC,0xFFAC5500,0xFF520000,0xFFAC5500,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFF520000,0xFF520000,0xFFACAAAC,0xFFACAAAC,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFF520000,0xFF520000,0xFFAC5500,0xFF520000,0xFF520000,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFF520000,0xFFACAAAC,0xFF520000,0xFFACAAAC,0xFFACAAAC,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFF520000,0xFF520000,0xFF520000,0xFFAC5500,0xFFAC5500,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFF520000,0xFF520000,0xFFACAAAC,0xFFACAAAC,0xFFAC5500,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFF520000,0xFFACAAAC,0xFF520000,0xFFAC5500,0xFF520000,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFF520000,0xFF520000,0xFFAC5500,0xFFACAAAC,0xFFAC5500,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFF520000,0xFF520000,0xFF520000,0xFFAC5500,0xFFACAAAC,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFF520000,0xFFACAAAC,0xFF520000,0xFF520000,0xFFAC5500,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFF520000,0xFFAC5500,0xFF520000,0xFFAC5500,0xFF520000,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFF520000,0xFF520000,0xFFACAAAC,0xFF520000,0xFFACAAAC,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFF520000,0xFFACAAAC,0xFFAC5500,0xFF520000,0xFFAC5500,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFF520000,0xFF520000,0xFF520000,0xFFACAAAC,0xFFACAAAC,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFF520000,0xFFACAAAC,0xFF520000,0xFFAC5500,0xFFACAAAC,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFF520000,0xFFAC5500,0xFF520000,0xFFAC5500,0xFF520000,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFF520000,0xFF520000,0xFFACAAAC,0xFFAC5500,0xFFACAAAC,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFF520000,0xFF520000,0xFFAC5500,0xFFAC5500,0xFFAC5500,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFF520000,0xFF520000,0xFFACAAAC,0xFFAC5500,0xFFACAAAC,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFF520000,0xFF520000,0xFF520000,0xFF520000,0xFFAC5500,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFF520000,0xFFACAAAC,0xFF520000,0xFFACAAAC,0xFFAC5500,0xFFAC5500,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFF520000,0xFFACAAAC,0xFF520000,0xFF520000,0xFF520000,0xFF520000,0xFFACAAAC,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFF520000,0xFF520000,0xFF520000,0xFFAC5500,0xFF520000,0xFFACAAAC,0xFFAC5500,0xFFAC5500,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFF520000,0xFFAC5500,0xFF520000,0xFF520000,0xFFAC5500,0xFFAC5500,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFF520000,0xFFFFFFFF,0xFFFFFFFF,0xFF520000,0xFFAC5500,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFF520000,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,0xFFFFFFFF,
];

pub(crate) const COCONUT: Bitmap = Bitmap {
    data: &COCONUT_DATA,
    w: 30,
    h: 80,
};