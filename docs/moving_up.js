/* @ts-self-types="./moving_up.d.ts" */

export class Game {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Game.prototype);
        obj.__wbg_ptr = ptr;
        GameFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        GameFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_game_free(ptr, 0);
    }
    /**
     * @param {string} id
     * @returns {string}
     */
    class(id) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ptr0 = passStringToWasm0(id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.game_class(this.__wbg_ptr, ptr0, len0);
            deferred2_0 = ret[0];
            deferred2_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {string} id
     * @returns {string}
     */
    record(id) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ptr0 = passStringToWasm0(id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.game_record(this.__wbg_ptr, ptr0, len0);
            deferred2_0 = ret[0];
            deferred2_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @param {string} id
     * @returns {string}
     */
    show(id) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ptr0 = passStringToWasm0(id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.game_show(this.__wbg_ptr, ptr0, len0);
            deferred2_0 = ret[0];
            deferred2_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @returns {string}
     */
    tooltip() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.game_tooltip(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @returns {string}
     */
    get id() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.__wbg_get_game_id(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {string} arg0
     */
    set id(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_game_id(this.__wbg_ptr, ptr0, len0);
    }
}
if (Symbol.dispose) Game.prototype[Symbol.dispose] = Game.prototype.free;

export class GamePerspectives {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(GamePerspectives.prototype);
        obj.__wbg_ptr = ptr;
        GamePerspectivesFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        GamePerspectivesFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_gameperspectives_free(ptr, 0);
    }
    /**
     * @returns {Game}
     */
    get game() {
        const ret = wasm.__wbg_get_gameperspectives_game(this.__wbg_ptr);
        return Game.__wrap(ret);
    }
    /**
     * @returns {string[]}
     */
    get perspectives() {
        const ret = wasm.__wbg_get_gameperspectives_perspectives(this.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @param {Game} arg0
     */
    set game(arg0) {
        _assertClass(arg0, Game);
        var ptr0 = arg0.__destroy_into_raw();
        wasm.__wbg_set_gameperspectives_game(this.__wbg_ptr, ptr0);
    }
    /**
     * @param {string[]} arg0
     */
    set perspectives(arg0) {
        const ptr0 = passArrayJsValueToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_gameperspectives_perspectives(this.__wbg_ptr, ptr0, len0);
    }
}
if (Symbol.dispose) GamePerspectives.prototype[Symbol.dispose] = GamePerspectives.prototype.free;

export class Ranking {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Ranking.prototype);
        obj.__wbg_ptr = ptr;
        RankingFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        RankingFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_ranking_free(ptr, 0);
    }
    /**
     * @returns {Team[]}
     */
    get teams() {
        const ret = wasm.__wbg_get_ranking_teams(this.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @param {string} id
     * @returns {number | undefined}
     */
    get_ranking(id) {
        const ptr0 = passStringToWasm0(id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.ranking_get_ranking(this.__wbg_ptr, ptr0, len0);
        return ret === 0x100000001 ? undefined : ret;
    }
    /**
     * @param {Sport} sport
     * @returns {string}
     */
    scoreboard_today(sport) {
        let deferred1_0;
        let deferred1_1;
        try {
            _assertClass(sport, Sport);
            const ret = wasm.ranking_scoreboard_today(this.__wbg_ptr, sport.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @param {Sport} sport
     * @returns {string[]}
     */
    scoreboard_urls(sport) {
        _assertClass(sport, Sport);
        const ret = wasm.ranking_scoreboard_urls(this.__wbg_ptr, sport.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @param {Team[]} arg0
     */
    set teams(arg0) {
        const ptr0 = passArrayJsValueToWasm0(arg0, wasm.__wbindgen_malloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_ranking_teams(this.__wbg_ptr, ptr0, len0);
    }
}
if (Symbol.dispose) Ranking.prototype[Symbol.dispose] = Ranking.prototype.free;

export class RankingType {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(RankingType.prototype);
        obj.__wbg_ptr = ptr;
        RankingTypeFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        RankingTypeFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_rankingtype_free(ptr, 0);
    }
    /**
     * @param {Teams} teams
     * @param {string} res
     */
    add_specific(teams, res) {
        _assertClass(teams, Teams);
        const ptr0 = passStringToWasm0(res, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.rankingtype_add_specific(this.__wbg_ptr, teams.__wbg_ptr, ptr0, len0);
    }
    /**
     * @param {Teams} teams
     * @param {string} res
     */
    add_weeks(teams, res) {
        _assertClass(teams, Teams);
        const ptr0 = passStringToWasm0(res, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.rankingtype_add_weeks(this.__wbg_ptr, teams.__wbg_ptr, ptr0, len0);
    }
    /**
     * @returns {string}
     */
    get_name() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.rankingtype_get_name(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @returns {Ranking}
     */
    get_ranking() {
        const ret = wasm.rankingtype_get_ranking(this.__wbg_ptr);
        return Ranking.__wrap(ret);
    }
    /**
     * @returns {string}
     */
    get_slug() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.rankingtype_get_slug(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @returns {string}
     */
    get_url1() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.rankingtype_get_url1(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @returns {string}
     */
    get_url2() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.rankingtype_get_url2(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @returns {boolean}
     */
    hide_points() {
        const ret = wasm.rankingtype_hide_points(this.__wbg_ptr);
        return ret !== 0;
    }
    /**
     * @param {Sport} sport
     * @returns {RankingType[]}
     */
    static options(sport) {
        _assertClass(sport, Sport);
        const ret = wasm.rankingtype_options(sport.__wbg_ptr);
        var v1 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
        wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
        return v1;
    }
    /**
     * @returns {boolean}
     */
    ranking_ready() {
        const ret = wasm.rankingtype_ranking_ready(this.__wbg_ptr);
        return ret !== 0;
    }
}
if (Symbol.dispose) RankingType.prototype[Symbol.dispose] = RankingType.prototype.free;

export class Sport {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Sport.prototype);
        obj.__wbg_ptr = ptr;
        SportFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        SportFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_sport_free(ptr, 0);
    }
    /**
     * @returns {SportEnum}
     */
    get sport() {
        const ret = wasm.__wbg_get_sport_sport(this.__wbg_ptr);
        return ret;
    }
    /**
     * @param {SportEnum} arg0
     */
    set sport(arg0) {
        wasm.__wbg_set_sport_sport(this.__wbg_ptr, arg0);
    }
    /**
     * @param {string} name
     * @returns {Sport}
     */
    static from_name(name) {
        const ptr0 = passStringToWasm0(name, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.sport_from_name(ptr0, len0);
        return Sport.__wrap(ret);
    }
}
if (Symbol.dispose) Sport.prototype[Symbol.dispose] = Sport.prototype.free;

/**
 * @enum {0 | 1 | 2 | 3 | 4 | 5}
 */
export const SportEnum = Object.freeze({
    MensBasketball: 0, "0": "MensBasketball",
    WomensBasketball: 1, "1": "WomensBasketball",
    MensLacrosse: 2, "2": "MensLacrosse",
    WomensLacrosse: 3, "3": "WomensLacrosse",
    Baseball: 4, "4": "Baseball",
    Softball: 5, "5": "Softball",
});

export class Team {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Team.prototype);
        obj.__wbg_ptr = ptr;
        TeamFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    static __unwrap(jsValue) {
        if (!(jsValue instanceof Team)) {
            return 0;
        }
        return jsValue.__destroy_into_raw();
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        TeamFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_team_free(ptr, 0);
    }
    /**
     * @returns {number}
     */
    get first_votes() {
        const ret = wasm.__wbg_get_team_first_votes(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @returns {string}
     */
    get id() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.__wbg_get_team_id(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @returns {string}
     */
    get name() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.__wbg_get_team_name(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @returns {number}
     */
    get rank() {
        const ret = wasm.__wbg_get_team_rank(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @returns {string}
     */
    get record() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.__wbg_get_team_record(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
    /**
     * @returns {number}
     */
    get votes() {
        const ret = wasm.__wbg_get_team_votes(this.__wbg_ptr);
        return ret >>> 0;
    }
    /**
     * @param {number} arg0
     */
    set first_votes(arg0) {
        wasm.__wbg_set_team_first_votes(this.__wbg_ptr, arg0);
    }
    /**
     * @param {string} arg0
     */
    set id(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_team_id(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @param {string} arg0
     */
    set name(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_team_name(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @param {number} arg0
     */
    set rank(arg0) {
        wasm.__wbg_set_team_rank(this.__wbg_ptr, arg0);
    }
    /**
     * @param {string} arg0
     */
    set record(arg0) {
        const ptr0 = passStringToWasm0(arg0, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        wasm.__wbg_set_team_record(this.__wbg_ptr, ptr0, len0);
    }
    /**
     * @param {number} arg0
     */
    set votes(arg0) {
        wasm.__wbg_set_team_votes(this.__wbg_ptr, arg0);
    }
    /**
     * @param {string} id
     * @returns {string}
     */
    static fix_id(id) {
        let deferred2_0;
        let deferred2_1;
        try {
            const ptr0 = passStringToWasm0(id, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len0 = WASM_VECTOR_LEN;
            const ret = wasm.team_fix_id(ptr0, len0);
            deferred2_0 = ret[0];
            deferred2_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred2_0, deferred2_1, 1);
        }
    }
    /**
     * @returns {string}
     */
    show_rank() {
        let deferred1_0;
        let deferred1_1;
        try {
            const ret = wasm.team_show_rank(this.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
}
if (Symbol.dispose) Team.prototype[Symbol.dispose] = Team.prototype.free;

export class Teams {
    static __wrap(ptr) {
        ptr = ptr >>> 0;
        const obj = Object.create(Teams.prototype);
        obj.__wbg_ptr = ptr;
        TeamsFinalization.register(obj, obj.__wbg_ptr, obj);
        return obj;
    }
    __destroy_into_raw() {
        const ptr = this.__wbg_ptr;
        this.__wbg_ptr = 0;
        TeamsFinalization.unregister(this);
        return ptr;
    }
    free() {
        const ptr = this.__destroy_into_raw();
        wasm.__wbg_teams_free(ptr, 0);
    }
    /**
     * @param {string} json
     * @returns {Teams}
     */
    static get_teams(json) {
        const ptr0 = passStringToWasm0(json, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        const len0 = WASM_VECTOR_LEN;
        const ret = wasm.teams_get_teams(ptr0, len0);
        return Teams.__wrap(ret);
    }
    /**
     * @param {Sport} sport
     * @returns {string}
     */
    static teams_url(sport) {
        let deferred1_0;
        let deferred1_1;
        try {
            _assertClass(sport, Sport);
            const ret = wasm.teams_teams_url(sport.__wbg_ptr);
            deferred1_0 = ret[0];
            deferred1_1 = ret[1];
            return getStringFromWasm0(ret[0], ret[1]);
        } finally {
            wasm.__wbindgen_free(deferred1_0, deferred1_1, 1);
        }
    }
}
if (Symbol.dispose) Teams.prototype[Symbol.dispose] = Teams.prototype.free;

/**
 * @param {number} left
 * @param {number} right
 * @returns {number}
 */
export function add(left, right) {
    const ret = wasm.add(left, right);
    return ret >>> 0;
}

/**
 * @param {Sport} sport
 * @param {Ranking} ranked_teams
 * @param {number} timezone_offset
 * @param {string} json
 * @returns {GamePerspectives[]}
 */
export function get_scores(sport, ranked_teams, timezone_offset, json) {
    _assertClass(sport, Sport);
    _assertClass(ranked_teams, Ranking);
    const ptr0 = passStringToWasm0(json, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    const len0 = WASM_VECTOR_LEN;
    const ret = wasm.get_scores(sport.__wbg_ptr, ranked_teams.__wbg_ptr, timezone_offset, ptr0, len0);
    var v2 = getArrayJsValueFromWasm0(ret[0], ret[1]).slice();
    wasm.__wbindgen_free(ret[0], ret[1] * 4, 4);
    return v2;
}

export function init_panics() {
    wasm.init_panics();
}

function __wbg_get_imports() {
    const import0 = {
        __proto__: null,
        __wbg___wbindgen_string_get_3e5751597f39a112: function(arg0, arg1) {
            const obj = arg1;
            const ret = typeof(obj) === 'string' ? obj : undefined;
            var ptr1 = isLikeNone(ret) ? 0 : passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            var len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg___wbindgen_throw_39bc967c0e5a9b58: function(arg0, arg1) {
            throw new Error(getStringFromWasm0(arg0, arg1));
        },
        __wbg_error_a6fa202b58aa1cd3: function(arg0, arg1) {
            let deferred0_0;
            let deferred0_1;
            try {
                deferred0_0 = arg0;
                deferred0_1 = arg1;
                console.error(getStringFromWasm0(arg0, arg1));
            } finally {
                wasm.__wbindgen_free(deferred0_0, deferred0_1, 1);
            }
        },
        __wbg_gameperspectives_new: function(arg0) {
            const ret = GamePerspectives.__wrap(arg0);
            return ret;
        },
        __wbg_getTime_9429d05129287418: function(arg0) {
            const ret = arg0.getTime();
            return ret;
        },
        __wbg_getTimezoneOffset_eeb05f23347950d1: function(arg0) {
            const ret = arg0.getTimezoneOffset();
            return ret;
        },
        __wbg_log_2d76c7db2656fd6d: function(arg0, arg1) {
            console.log(getStringFromWasm0(arg0, arg1));
        },
        __wbg_new_0_a719938e6f92ddf4: function() {
            const ret = new Date();
            return ret;
        },
        __wbg_new_223944ca37c18764: function(arg0) {
            const ret = new Date(arg0);
            return ret;
        },
        __wbg_new_227d7c05414eb861: function() {
            const ret = new Error();
            return ret;
        },
        __wbg_rankingtype_new: function(arg0) {
            const ret = RankingType.__wrap(arg0);
            return ret;
        },
        __wbg_stack_3b0d974bbf31e44f: function(arg0, arg1) {
            const ret = arg1.stack;
            const ptr1 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
            const len1 = WASM_VECTOR_LEN;
            getDataViewMemory0().setInt32(arg0 + 4 * 1, len1, true);
            getDataViewMemory0().setInt32(arg0 + 4 * 0, ptr1, true);
        },
        __wbg_team_new: function(arg0) {
            const ret = Team.__wrap(arg0);
            return ret;
        },
        __wbg_team_unwrap: function(arg0) {
            const ret = Team.__unwrap(arg0);
            return ret;
        },
        __wbindgen_cast_0000000000000001: function(arg0) {
            // Cast intrinsic for `F64 -> Externref`.
            const ret = arg0;
            return ret;
        },
        __wbindgen_cast_0000000000000002: function(arg0, arg1) {
            // Cast intrinsic for `Ref(String) -> Externref`.
            const ret = getStringFromWasm0(arg0, arg1);
            return ret;
        },
        __wbindgen_init_externref_table: function() {
            const table = wasm.__wbindgen_externrefs;
            const offset = table.grow(4);
            table.set(0, undefined);
            table.set(offset + 0, undefined);
            table.set(offset + 1, null);
            table.set(offset + 2, true);
            table.set(offset + 3, false);
        },
    };
    return {
        __proto__: null,
        "./moving_up_bg.js": import0,
    };
}

const GameFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_game_free(ptr >>> 0, 1));
const GamePerspectivesFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_gameperspectives_free(ptr >>> 0, 1));
const RankingFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_ranking_free(ptr >>> 0, 1));
const RankingTypeFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_rankingtype_free(ptr >>> 0, 1));
const SportFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_sport_free(ptr >>> 0, 1));
const TeamFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_team_free(ptr >>> 0, 1));
const TeamsFinalization = (typeof FinalizationRegistry === 'undefined')
    ? { register: () => {}, unregister: () => {} }
    : new FinalizationRegistry(ptr => wasm.__wbg_teams_free(ptr >>> 0, 1));

function addToExternrefTable0(obj) {
    const idx = wasm.__externref_table_alloc();
    wasm.__wbindgen_externrefs.set(idx, obj);
    return idx;
}

function _assertClass(instance, klass) {
    if (!(instance instanceof klass)) {
        throw new Error(`expected instance of ${klass.name}`);
    }
}

function getArrayJsValueFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    const mem = getDataViewMemory0();
    const result = [];
    for (let i = ptr; i < ptr + 4 * len; i += 4) {
        result.push(wasm.__wbindgen_externrefs.get(mem.getUint32(i, true)));
    }
    wasm.__externref_drop_slice(ptr, len);
    return result;
}

let cachedDataViewMemory0 = null;
function getDataViewMemory0() {
    if (cachedDataViewMemory0 === null || cachedDataViewMemory0.buffer.detached === true || (cachedDataViewMemory0.buffer.detached === undefined && cachedDataViewMemory0.buffer !== wasm.memory.buffer)) {
        cachedDataViewMemory0 = new DataView(wasm.memory.buffer);
    }
    return cachedDataViewMemory0;
}

function getStringFromWasm0(ptr, len) {
    ptr = ptr >>> 0;
    return decodeText(ptr, len);
}

let cachedUint8ArrayMemory0 = null;
function getUint8ArrayMemory0() {
    if (cachedUint8ArrayMemory0 === null || cachedUint8ArrayMemory0.byteLength === 0) {
        cachedUint8ArrayMemory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachedUint8ArrayMemory0;
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

function passArrayJsValueToWasm0(array, malloc) {
    const ptr = malloc(array.length * 4, 4) >>> 0;
    for (let i = 0; i < array.length; i++) {
        const add = addToExternrefTable0(array[i]);
        getDataViewMemory0().setUint32(ptr + 4 * i, add, true);
    }
    WASM_VECTOR_LEN = array.length;
    return ptr;
}

function passStringToWasm0(arg, malloc, realloc) {
    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length, 1) >>> 0;
        getUint8ArrayMemory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len, 1) >>> 0;

    const mem = getUint8ArrayMemory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }
    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3, 1) >>> 0;
        const view = getUint8ArrayMemory0().subarray(ptr + offset, ptr + len);
        const ret = cachedTextEncoder.encodeInto(arg, view);

        offset += ret.written;
        ptr = realloc(ptr, len, offset, 1) >>> 0;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
cachedTextDecoder.decode();
const MAX_SAFARI_DECODE_BYTES = 2146435072;
let numBytesDecoded = 0;
function decodeText(ptr, len) {
    numBytesDecoded += len;
    if (numBytesDecoded >= MAX_SAFARI_DECODE_BYTES) {
        cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });
        cachedTextDecoder.decode();
        numBytesDecoded = len;
    }
    return cachedTextDecoder.decode(getUint8ArrayMemory0().subarray(ptr, ptr + len));
}

const cachedTextEncoder = new TextEncoder();

if (!('encodeInto' in cachedTextEncoder)) {
    cachedTextEncoder.encodeInto = function (arg, view) {
        const buf = cachedTextEncoder.encode(arg);
        view.set(buf);
        return {
            read: arg.length,
            written: buf.length
        };
    };
}

let WASM_VECTOR_LEN = 0;

let wasmModule, wasm;
function __wbg_finalize_init(instance, module) {
    wasm = instance.exports;
    wasmModule = module;
    cachedDataViewMemory0 = null;
    cachedUint8ArrayMemory0 = null;
    wasm.__wbindgen_start();
    return wasm;
}

async function __wbg_load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);
            } catch (e) {
                const validResponse = module.ok && expectedResponseType(module.type);

                if (validResponse && module.headers.get('Content-Type') !== 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve Wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else { throw e; }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);
    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };
        } else {
            return instance;
        }
    }

    function expectedResponseType(type) {
        switch (type) {
            case 'basic': case 'cors': case 'default': return true;
        }
        return false;
    }
}

function initSync(module) {
    if (wasm !== undefined) return wasm;


    if (module !== undefined) {
        if (Object.getPrototypeOf(module) === Object.prototype) {
            ({module} = module)
        } else {
            console.warn('using deprecated parameters for `initSync()`; pass a single object instead')
        }
    }

    const imports = __wbg_get_imports();
    if (!(module instanceof WebAssembly.Module)) {
        module = new WebAssembly.Module(module);
    }
    const instance = new WebAssembly.Instance(module, imports);
    return __wbg_finalize_init(instance, module);
}

async function __wbg_init(module_or_path) {
    if (wasm !== undefined) return wasm;


    if (module_or_path !== undefined) {
        if (Object.getPrototypeOf(module_or_path) === Object.prototype) {
            ({module_or_path} = module_or_path)
        } else {
            console.warn('using deprecated parameters for the initialization function; pass a single object instead')
        }
    }

    if (module_or_path === undefined) {
        module_or_path = new URL('moving_up_bg.wasm', import.meta.url);
    }
    const imports = __wbg_get_imports();

    if (typeof module_or_path === 'string' || (typeof Request === 'function' && module_or_path instanceof Request) || (typeof URL === 'function' && module_or_path instanceof URL)) {
        module_or_path = fetch(module_or_path);
    }

    const { instance, module } = await __wbg_load(await module_or_path, imports);

    return __wbg_finalize_init(instance, module);
}

export { initSync, __wbg_init as default };
