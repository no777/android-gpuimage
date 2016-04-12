#pragma version(1)
#pragma rs java_package_name( tv.lycam.rtmp.video)

rs_allocation gIn;

int width;
int height;
int frameSize;

void yuvToRgb(const uchar *v_in, uchar4 *v_out, const void *usrData, uint32_t x, uint32_t y) {

    uchar yp = rsGetElementAtYuv_uchar_Y(gIn, x, y) & 0xFF;

    int index = frameSize + (x & (~1)) + (( y>>1) * width );
    int v = (int)( rsGetElementAt_uchar(gIn, index) & 0xFF ) -128;
    int u = (int)( rsGetElementAt_uchar(gIn, index+1) & 0xFF ) -128;

    int r = (int) (1.164f * yp  + 1.596f * v );
    int g = (int) (1.164f * yp  - 0.813f * v  - 0.391f * u);
    int b = (int) (1.164f * yp  + 2.018f * u );

    r = r>255? 255 : r<0 ? 0 : r;
    g = g>255? 255 : g<0 ? 0 : g;
    b = b>255? 255 : b<0 ? 0 : b;

    uchar4 res4;
    res4.r = (uchar)r;
    res4.g = (uchar)g;
    res4.b = (uchar)b;
    res4.a = 0xFF;

    *v_out = res4;
}