import { DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuTrigger } from '@/components/ui/dropdown-menu'
import {
  SidebarGroup,
  SidebarGroupLabel,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
  useSidebar,
} from '@/components/ui/sidebar'
import { useGetProjectsByIdQuery } from '@/hooks'
import { FolderPlusIcon, FolderSyncIcon, FolderTreeIcon, MoreHorizontal } from 'lucide-react'
import { useNavigate } from 'react-router-dom'

export const NavProjects = () => {
  const navigate = useNavigate()
  const { isMobile } = useSidebar()
  const { data: projectsById, isPending: isPendingGetProjectsById } = useGetProjectsByIdQuery()

  return (
    <SidebarGroup className="group-data-[collapsible=icon]:hidden">
      <SidebarGroupLabel>Projects</SidebarGroupLabel>
      <SidebarMenu>
        <SidebarMenuItem>
          <DropdownMenu>
            <DropdownMenuTrigger asChild>
              <SidebarMenuButton className="data-[state=open]:bg-sidebar-accent data-[state=open]:text-sidebar-accent-foreground">
                {!isPendingGetProjectsById ? (
                  <>
                    <FolderTreeIcon />
                    <span>{projectsById?.name}</span>
                  </>
                ) : (
                  <TitleLoader />
                )}{' '}
                <MoreHorizontal className="ml-auto" />
              </SidebarMenuButton>
            </DropdownMenuTrigger>
            <DropdownMenuContent className="w-48" side={isMobile ? 'bottom' : 'right'} align={isMobile ? 'end' : 'start'}>
              <DropdownMenuItem
                onClick={() => {
                  navigate('/projects')
                }}
              >
                <FolderSyncIcon />
                <span>Change Project</span>
              </DropdownMenuItem>
              <DropdownMenuItem
                onClick={() => {
                  navigate('/projects/create')
                }}
              >
                <FolderPlusIcon />
                <span>Create Project</span>
              </DropdownMenuItem>
            </DropdownMenuContent>
          </DropdownMenu>
        </SidebarMenuItem>
      </SidebarMenu>
    </SidebarGroup>
  )
}

const TitleLoader = () => {
  return (
    <div className="w-28 animate-pulse">
      <div className="space-y-1">
        <div className="grid grid-cols-3 gap-4">
          <div className="col-span-2 h-2 rounded bg-slate-200" />
        </div>
        <div className="h-2 rounded bg-slate-200" />
      </div>
    </div>
  )
}
