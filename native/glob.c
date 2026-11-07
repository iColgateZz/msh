#include <glob.h>
#include <stddef.h>

int msh_glob(const char *pattern, void (*emit)(const char *, void *), void *context)
{
    glob_t matches = {0};
    int status = glob(pattern, GLOB_ERR | GLOB_NOCHECK, NULL, &matches);

    if (status == 0) {
        for (size_t index = 0; index < matches.gl_pathc; index++) {
            emit(matches.gl_pathv[index], context);
        }
    }

    globfree(&matches);

    if (status == 0 || status == GLOB_NOMATCH) {
        return 0;
    }

    return status == GLOB_NOSPACE ? 1 : 2;
}
