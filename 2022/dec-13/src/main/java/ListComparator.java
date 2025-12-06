import java.util.Arrays;
import java.util.Comparator;
import java.util.List;

public class ListComparator implements Comparator<Object> {

    @Override
    public int compare(Object lhs, Object rhs) {
        int res;
        if(lhs instanceof Double && rhs instanceof Double) {
            if((Double)lhs < (Double)rhs) return -1;
            if((Double)lhs > (Double)rhs) return 1;
            else return 0;
        } else if(lhs instanceof Double && rhs instanceof List) {
            return compare(Arrays.asList(lhs), rhs);
        } else if(lhs instanceof List && rhs instanceof Double) {
            return compare(lhs, Arrays.asList(rhs));
        } else if(lhs instanceof List && rhs instanceof List) {
            int i = 0;

            for(; i < ((List<?>) lhs).size(); i++) {
                if(i >= ((List) rhs).size()) { return 1; }
                res = compare(((List<?>) lhs).get(i), ((List<?>) rhs).get(i));
                if(res != 0) { return res; }
            }

            return ((List) lhs).size() == ((List) rhs).size() ? 0 : -1;
        }

        return 1;
    }
}
