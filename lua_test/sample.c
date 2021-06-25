#include <stdio.h>

#include "lua.h"
#include "lualib.h"
#include "lauxlib.h"

int main(void) {
    lua_State *L = luaL_newstate();
    luaL_openlibs(L);
    if (luaL_loadfile(L, "sample.lua") || lua_pcall(L, 0, 0, 0)) {
        printf("sample.lua cannot open.\n");
        return 1;
    }
    // process

    lua_close(L);
    return 0;
}
