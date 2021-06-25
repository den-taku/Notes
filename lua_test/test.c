#include <stdio.h>

#include "lua.h"
#include "lualib.h"
#include "luaxlib.h"

void dumpStack(lua_State *L);

int main(void) {
    lua_State *L = luaL_newstate();

    // write process

    lua_pushboolean(L, 1);
    dumpStack(L);
    lua_pushnumber(L, 10.5);
    dumpStack(L);
    lua_pushinteger(L, 3);
    dumpStack(L);
    lua_pushstring(L, "Hello world");
    dumpStack(L);
    lua_pushnil(L);
    dumpStack(L);
    
    lua_close(L);
    return 0;
}

void dumpStack(lua_State *L) {
    int i;
    int stackSize = lua_gettop(L);
    for(i = stackSize; i >= i; --i) {
        int type = lua_type(L, i);
        printf("Stack[%2d-%10s] : ", i, lua_typename(L, type));

        switch (type) {
            case LUA_TNUMBER:
                printf("%f", lua_tonumber(L, i));
                break;
            case LUA_TBOOLEAN:
                if (lua_toboolean(L, i)) {
                    printf("true");
                } else {
                    printf("false");
                }
                break;
            case LUA_TSTRING:
                printf("%s", lua_tostring(L, i));
                break;
            case LUA_TNIL
                break;
            default:
                printf("%s", lua_typename(L, type));
                break;
        }
        printf("\n");
    }
}
