package me.nettee.brene.dom;

import java.util.HashMap;
import java.util.Map;
import java.util.Optional;

public class AttrMap {

    private Map<String, String> attrMap = new HashMap<>();

    Optional<String> get(String key) {
        if (attrMap.containsKey(key)) {
            return Optional.of(attrMap.get(key));
        } else {
            return Optional.empty();
        }
    }

    public void insert(String key, String value) {
        attrMap.put(key, value);
    }
}
