import { Icon,faPlus } from "@rivet-gg/icons";
import {
  Button,
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
  Code,
  Flex,
  H4,
  Link as RivetLink,
  Separator,
  SidebarNavigation,
  SidebarPageContent,
} from "@rivet-gg/components";
import { useSuspenseQuery } from "@tanstack/react-query";
import { createFileRoute, Link, useLocation, useNavigate } from "@tanstack/react-router";
import { useContext, useLayoutEffect } from "react";
import { MessageBanner } from "../components/message-banner";
import { ModuleCard } from "../components/module-card";
import { ModuleSearchContext } from "../components/module-search-context";
import { ModulesConfigForm } from "../components/modules-config-form";
import { NewModuleCard } from "../components/new-module-card";
import { projectManifestQueryOptions } from "../queries";

function NewModuleButton() {
  const ref = useContext(ModuleSearchContext);
  return (
    <Button
      size="sm"
      className="text-foreground"
      startIcon={<Icon icon={faPlus} />}
      variant="outline"
      onClick={() => ref?.current?.click()}
    >
      Add module
    </Button>
  );
}

function IndexRoute() {
  const {hash} = useLocation();
  const navigate = useNavigate();
  const { data } = useSuspenseQuery(projectManifestQueryOptions());

  useLayoutEffect(() => {
    if(!hash) {
      return;
    }

    window.document.getElementById(hash)?.scrollIntoView({ behavior: 'smooth', block: 'start', inline: 'start'});
    const timeout = setTimeout(() => {
      navigate({ hash: false });
    }, 1000);

    return () => {
      clearTimeout(timeout);
    }
  }, [hash]);

  return (
    <SidebarPageContent
      sidebar={
        <SidebarNavigation>
          <H4 className="text-foreground">Modules</H4>
          <NewModuleButton />
          {Object.entries(data?.modules).map(([name, module]) => {
            return (
              <Link to="/" hash={name} key={name} className="hover:text-foreground transition-colors">
                {module.config.icon ? <Icon icon={module.config.icon} className="mr-1" /> : null}
                {module.config.name || module.namePascal}
              </Link>
            );
          })}
        </SidebarNavigation>
      }
    >
      <Flex direction="col" items="start" gap="4">
        <NewModuleCard />
        <Separator my="4" />
        <ModulesConfigForm {...data}>
          <MessageBanner />
          {Object.entries(data?.modules).map(([name, module]) => (
            <ModuleCard
              key={name}
              module={module}
              dependants={Object.values(data.modules).filter(
                (m) => m.config.dependencies?.[name] !== undefined,
              )}
              isHighlighted={hash === name}
              isRegistryExternal={data.registries[module.registryName]?.isExternal}
            />
          ))}
        </ModulesConfigForm>
        <Card w="full" className="bg-background-main">
          <CardHeader>
            <CardTitle>Build your own module</CardTitle>
          </CardHeader>
          <CardContent>
            <Flex gap="2">
              <Button variant="outline" asChild>
                <a
                  href="https://rivet.gg/docs/general/modules/build/overview"
                  target="_blank"
                  rel="noreferrer"
                >
                  Documentation
                </a>
              </Button>
              <Button variant="outline" asChild>
                <a
                  href="https://github.com/rivet-gg/toolchain/issues/new"
                  target="_blank"
                  rel="noreferrer"
                >
                  Request module
                </a>
              </Button>
            </Flex>
          </CardContent>
        </Card>
      </Flex>
    </SidebarPageContent>
  );
}

export const Route = createFileRoute("/")({
  component: IndexRoute,
  pendingComponent: () => (
    <Card>
      <CardHeader>
        <CardTitle>Waiting for Rivet server...</CardTitle>
        <CardDescription>
          Taking longer than expected? Take a look at the terminal, maybe something's wrong.
        </CardDescription>
      </CardHeader>
    </Card>
  ),
  errorComponent: ({ error }) => (
    <Card>
      <CardHeader>
        <CardTitle>Uh oh. There was an error.</CardTitle>
        <CardDescription>
          This page has lost connection to Rivet local server. Try refreshing the page, or take a look at the terminal
          for more information. In case the issue still persist, create an issue on{" "}
          <RivetLink href="https://github.com/rivet-gg/toolchain/issues/new">
            GitHub
          </RivetLink>{" "}
          or <RivetLink href="https://rivet.gg/discord">contact us</RivetLink>!
        </CardDescription>
      </CardHeader>
      <CardContent>
        <Code>
          {"toString" in error ? error.toString() : JSON.stringify(error)}
        </Code>
      </CardContent>
    </Card>
  ),
});
