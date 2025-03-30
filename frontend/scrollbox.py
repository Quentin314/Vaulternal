import pygame

BORDER_SIZE = 2
BORDER_COLOR = "#FFFFFF"

class ScrollBox:
    def __init__(self, x, y, width, height):
        self.x = x
        self.y = y
        self.width = width
        self.height = height
        
        self.elem_height = 40
        self.padding = 10
        self.font = pygame.font.Font(None, self.elem_height - self.padding)

        self.bg_color = "#04080F"
        self.elem_color = "#507DBC"
        self.hover_color = "#406DAC"
        self.selected_color = "#DAE3E5"
        self.text_color = (0, 0, 0)

        self.max_elems = -1

        self.files = []
        self.selected = 0 # Selected element index
        self.first = 0    # First visible element index

        self.active = False

    def draw(self, screen):
        if not self.active:
            return
        
        pygame.draw.rect(screen, self.bg_color, pygame.Rect((self.x, self.y), (self.width, self.height)))
        pygame.draw.rect(screen, BORDER_COLOR, pygame.Rect((self.x, self.y), (self.width, self.height)), BORDER_SIZE)

        max_elem_count = (self.height - self.padding * 2) // self.elem_height
        last = min(len(self.files), self.first + max_elem_count - 1)

        for id in range(self.first, last):
            file = self.files[id]
            color = self.selected_color if id == self.selected else self.elem_color
            
            pos = (self.x + self.padding, self.y + self.padding + (id-self.first) * (self.elem_height + self.padding))
            size = (self.width - self.padding*2, self.elem_height)
            pygame.draw.rect(screen, color, pygame.Rect(pos, size))
            
            cross_pos = (self.x + self.width - self.elem_height - self.padding + 4, pos[1] + 4)
            cross_size = (self.elem_height - 8, self.elem_height - 8)
            pygame.draw.rect(screen, (255, 100, 100), pygame.Rect(cross_pos, cross_size))
            center = (self.x + self.width - self.elem_height//2 - self.padding, pos[1] + self.elem_height//2)
            d = self.elem_height//4
            pygame.draw.line(screen, (255, 255, 255), (center[0] - d, center[1] - d), (center[0] + d, center[1] + d), 4)
            pygame.draw.line(screen, (255, 255, 255), (center[0] - d, center[1] + d), (center[0] + d, center[1] - d), 4)

            name = file.name
            i = len(name)
            while self.font.size(name[:i])[0] >= self.width - self.elem_height:
                i -= 1

            text = self.font.render(name[:i], True, (0, 0, 0))
            text_pos = (pos[0] + self.padding, pos[1] + self.padding)
            screen.blit(text, text_pos)
    
    def add_file(self, name, path):
        if self.max_elems > -1 and len(self.files) < self.max_elems:
            return
        new_file = File(name, path)
        self.files.append(new_file)
        self.selected = len(self.files) - 1
    
    def clear_files(self):
        self.files = []

    def click(self, mouse_pos):
        mx, my = mouse_pos
        if not (self.x < mx < self.x + self.width and self.y < my < self.y + self.height):
            return
        
        elem = (my - self.y) // (self.elem_height + self.padding) + self.first

        if elem < len(self.files) and mx > self.x + self.width - self.elem_height - self.padding:
            self.files.pop(elem)
            return "deleted"
        elif elem < len(self.files) and elem != self.selected:
            self.selected = elem
            file = self.files[self.selected]
            return file

    def scroll(self, direction):
        max_elem_count = (self.height - self.padding * 2) // self.elem_height
        if self.first + direction < 0 or self.first + direction + max_elem_count > len(self.files):
            return
        self.first += direction

class File:
    def __init__(self, name, path=None):
        self.name = name
        self.path = path