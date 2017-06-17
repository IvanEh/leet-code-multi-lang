import com.gmail.at.ivanehreshi.humming;

public class Solution {
    public int hammingDistance(int x, int y) {
        int dist = 0;

        while (x != 0 || y != 0) {
            if (isFirstBitDiffers(x, y)) {
                dist++;
            }

            x >>= 1;
            y >>= 1;
        }
        
        return dist;
    }

    private int firstBit(int number) {
        return number & 1;
    }

    private boolean isFirstBitDiffers(int x, int y) {
        return firstBit(x) != firstBit(y);
    }
}