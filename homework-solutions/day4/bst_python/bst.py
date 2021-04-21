
class BST:
    """
    Binary search tree
    """
    def __init__(self, key=None, value=None):
        self.key = key
        self.value = value
        self.left = None
        self.right = None

    def search(self, key):
        if not self.key:
            return None
        if self.key == key:
            return self.value
        elif key < self.key:
            if self.left:
                return self.left.search(key)
            else:
                return None
        else:
            if self.right:
                return self.right.search(key)
            else:
                return None

    def insert(self, key, value):
        if not self.key:
            self.key = key
            self.value = value
        elif key == self.key:
            self.value = value
        elif key < self.key:
            if self.left:
                self.left.insert(key, value)
            else:
                self.left = BST(key, value)
        else:
            if self.right:
                self.right.insert(key, value)
            else:
                self.right = BST(key, value)

    def delete(self, key):
        if not self.key:
            raise KeyError()
        if key < self.key:
            if self.left:
                self.left.delete(key)
            else:
                raise KeyError()
        elif key > self.key:
            if self.right:
                self.right.delete(key)
            else:
                raise KeyError()
        else:
            # delete the key here
            if self.left and self.right:
                # exchange root node for min node on right side
                successor = self.right._find_min_node()
                self.key = successor.key
                self.value = successor.value
                self.right.delete(self.key)
            elif self.left:
                self.key = self.left.key
                self.value = self.left.value
                self.right = self.left.right
                self.left = self.left.left
                # garbage collector deletes previous self.left
            elif self.right:
                self.key = self.right.key
                self.value = self.right.value
                self.left = self.right.left
                self.right = self.right.right
                # garbage collector deletes previous self.right
            else:
                # just delete this node
                self.key = None
                self.value = None

    def _find_min_node(self):
        if not self.key:
            raise KeyError()
        if self.left:
            return self.left._find_min_node()
        else:
            return self

    def __iter__(self):
        # pre-order iteration
        if not self.key:
            return
        if self.left:
            yield from iter(self.left)
        yield (self.key, self.value)
        if self.right:
            yield from iter(self.right)


def test():
    b = BST()
    b.insert(5, "five")
    b.insert(3, "three")
    b.insert(7, "seven")
    b.insert(2, "two")
    b.insert(3, "three-three")
    b.insert(6, "six")

    assert b.search(5) == "five"
    assert b.search(2) == "two"
    assert b.search(6) == "six"

    b.delete(5)

    assert list(b) == [(2, "two"), (3, "three-three"), (6, "six"), (7, "seven")]

if __name__ == '__main__':
    test()
