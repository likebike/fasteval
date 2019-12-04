#include "tinyexpr.h"
#include <stdio.h>


int main(int argc, char *argv[]) {
    /* const char *EXPR = "(3 * (3 + 3) / 3)"; */
    /* const char *EXPR = "3 * 3 - 3 / 3"; */
    /* const char *EXPR = "2 ^ (3 ^ 4)"; */
    /* const char *EXPR = "x * 2"; */
    /* const char *EXPR = "sin(x)"; */
     const char *EXPR = "(-z + sqrt(z^2 - 4*x*y)) / (2*x)"; 
    /* const char *EXPR = "((((87))) - 73) + (97 + (((15 / 55 * ((31)) + 35))) + (15 - (9)) - (39 / 26) / 20 / 91 + 27 / (33 * 26 + 28 - (7) / 10 + 66 * 6) + 60 / 35 - ((29) - (69) / 44 / (92)) / (89) + 2 + 87 / 47 * ((2)) * 83 / 98 * 42 / (((67)) * ((97))) / (34 / 89 + 77) - 29 + 70 * (20)) + ((((((92))) + 23 * (98) / (95) + (((99) * (41))) + (5 + 41) + 10) - (36) / (6 + 80 * 52 + (90))))"; */


    double x=1;
    double y=2;
    double z=3;
    te_variable vars[] = {{"x",&x}, {"y",&y}, {"z",&z}};

    int err;
    te_expr *n = te_compile(EXPR, vars, 3, &err);
    if (!n) {
        printf("\t%*s^\nError near here", err-1, "");
    }

    int i;
    for(i=0; i<1000000000; i++) {
        /*
        double r = te_interp(EXPR, 0);
        */

        /*
        int err;
        te_expr *n = te_compile(EXPR, vars, 3, &err);
        if (n) {
            const double r = te_eval(n);  /* printf("Result:\n\t%f\n", r); * /
            te_free(n);
        } else {
            printf("\t%*s^\nError near here", err-1, "");
        }
        */


        const double r = te_eval(n);  /* printf("Result:\n\t%f\n", r); * /



        /* printf("The expression:\n\t%s\nevaluates to:\n\t%f\n", c, r); */
    }

    te_free(n);

    return 0;
}
